import { API_URL, ELECTION_MAP, RELEVANT_PARTIES } from "./config";
import { multiply, transpose, lusolve } from 'mathjs';
import { Polls, Query, Order, DataType } from 'german-election-polls';
import type { Survey } from 'german-election-polls/lib/models/Survey';
// @ts-ignore
import results_csv from '$lib/data/results_germany.csv';
import type { Election, ElectionResult, Result, PredictionResult } from "./types";


export async function get_available_elections(fetch: typeof window.fetch) {
    const endpoint = `${API_URL}/election`;
    const response = await fetch(endpoint);
    if (!response.ok) {
        throw new Error(`Failed to fetch elections: ${response.statusText}`);
    }

    const elections = await response.json();
    return elections["elections"];
}

export async function get_available_regions(fetch: typeof window.fetch) {
    const endpoint = `${API_URL}/region`;
    const response = await fetch(endpoint);
    if (!response.ok) {
        throw new Error(`Failed to fetch regions: ${response.statusText}`);
    }

    const regions = await response.json();
    return regions["regions"];
}

/**
* Gets the results for a specific region across all elections
*/
export function getResultsForRegion(selectedRegion: number, elections: Election[]): ElectionResult[] {
    let results = [];
    for (const election of elections.sort((a, b) => a.date - b.date)) {
        let voters = 0;
        let parties = new Map();

        for (const region of election.region) {
            if (selectedRegion == 0 || selectedRegion == region.id) {
                voters += region.turnout[0].voted;
                for (const vote of region.votes) {
                    if (RELEVANT_PARTIES.includes(vote.id)) {
                        if (!parties.has(vote.id)) {
                            parties.set(vote.id, 0);
                        }
                        parties.set(vote.id, parties.get(vote.id) + vote.votes);
                    }
                }
            }
        }

        for (const [id, votes] of parties) {
            parties.set(id, votes / voters);
        }

        const name =
            ELECTION_MAP[election["name"] as keyof typeof ELECTION_MAP] +
            String(election["date"]);
        const result: ElectionResult = { name, ...Object.fromEntries(parties) };
        results.push(result);
    }

    return results;
}

export const results_bund = getBundResults();
function getBundResults() {
    const results_bund: ElectionResult[] = results_csv.map((row: any) => ({
        ...row,
        AfD: parseFloat(row.AfD.toString().replace('%', '')) / 100,
        CDU: parseFloat(row.CDU.toString().replace('%', '')) / 100,
        FDP: parseFloat(row.FDP.toString().replace('%', '')) / 100,
        GRÜNE: parseFloat(row.GRÜNE.toString().replace('%', '')) / 100,
        LINKE: parseFloat(row.LINKE.toString().replace('%', '')) / 100,
        SPD: parseFloat(row.SPD.toString().replace('%', '')) / 100,
        Sonstige: parseFloat(row.Sonstige.toString().replace('%', '')) / 100,
    }));
    return results_bund;
}

const MIN_SURVEYS = 3;
// Progressively widened windows (days) -- stop at the first that yields at least
// MIN_SURVEYS. Keeps the normal case at 14 days, only reaches further back when
// polling is sparse (e.g. summer break, no upcoming election).
const SURVEY_WINDOW_DAYS = [14, 21, 28, 42, 60, 90];

async function getDawumSurveys(): Promise<{ surveys: Survey[]; windowDays: number }> {
    const polls = new Polls();
    await polls.update();

    for (const windowDays of SURVEY_WINDOW_DAYS) {
        const limit = new Date(Date.now() - 1000 * 60 * 60 * 24 * windowDays);
        const { surveys } = await polls.select([
            Query.include([DataType.Surveys]),
            Query.Survey.Release.isGreater(limit),
            Query.Survey.Parliament.Shortcut.is(['Bundestag']),
            Query.Survey.Sort.allResults(Order.Desc),
        ]);
        if (surveys.length >= MIN_SURVEYS) {
            return { surveys, windowDays };
        }
    }

    // Widest window still didn't reach MIN_SURVEYS -- return whatever we have
    // (possibly empty) rather than silently looping forever.
    const widest = SURVEY_WINDOW_DAYS[SURVEY_WINDOW_DAYS.length - 1];
    const limit = new Date(Date.now() - 1000 * 60 * 60 * 24 * widest);
    const { surveys } = await polls.select([
        Query.include([DataType.Surveys]),
        Query.Survey.Release.isGreater(limit),
        Query.Survey.Parliament.Shortcut.is(['Bundestag']),
        Query.Survey.Sort.allResults(Order.Desc),
    ]);
    return { surveys, windowDays: widest };
}

export async function getSurveyData(): Promise<ElectionResult> {
    const { surveys, windowDays } = await getDawumSurveys();
    if (surveys.length === 0) {
        throw new Error("Keine aktuellen Dawum-Umfragen gefunden (auch nach Ausweitung auf 90 Tage)");
    }

    // Weight by sample size (Fallzahl) and recency within the window used (Aktualität) --
    // larger and newer surveys count more, but nothing is weighted to zero.
    const now = Date.now();
    const weightOf = (s: Survey) => {
        const ageDays = (now - s.release.getTime()) / (1000 * 60 * 60 * 24);
        const recency = Math.max(0.2, 1 - ageDays / windowDays);
        return (s.participants ?? 1) * recency;
    };
    const totalWeight = surveys.reduce((sum, s) => sum + weightOf(s), 0);

    let mean: ElectionResult = {
        name: "Durchschnitt",
        CDU: 0,
        SPD: 0,
        GRÜNE: 0,
        LINKE: 0,
        FDP: 0,
        AfD: 0,
        Sonstige: 0,
    };

    for (const survey of surveys) {
        const weight = weightOf(survey) / totalWeight;
        mean.CDU += weight * (survey.results.find(r => r.party.shortcut === 'CDU/CSU')?.result ?? 0) / 100;
        mean.SPD += weight * (survey.results.find(r => r.party.shortcut === 'SPD')?.result ?? 0) / 100;
        mean.GRÜNE += weight * (survey.results.find(r => r.party.shortcut === 'Grüne')?.result ?? 0) / 100;
        mean.LINKE += weight * (survey.results.find(r => r.party.shortcut === 'Linke')?.result ?? 0) / 100;
        mean.FDP += weight * (survey.results.find(r => r.party.shortcut === 'FDP')?.result ?? 0) / 100;
        mean.AfD += weight * (survey.results.find(r => r.party.shortcut === 'AfD')?.result ?? 0) / 100;
    }
    // Residual, not a weighted average of each survey's own "Sonstige" line -- keeps this
    // consistent with how Sonstige is defined everywhere else (see toResult/predictResults).
    mean.Sonstige = 1 - (mean.CDU + mean.SPD + mean.GRÜNE + mean.LINKE + mean.FDP + mean.AfD);

    return mean;
}

// Fits y = intercept + coefs·x via OLS (normal equations) and returns [intercept, ...coefs].
function fitOLS(X: number[][], y: number[]): number[] {
    const design = X.map(row => [1, ...row]);
    const Xt = transpose(design);
    const XtX = multiply(Xt, design) as number[][];
    const Xty = multiply(Xt, y) as number[];
    const beta = lusolve(XtX, Xty) as number[][];
    return beta.map(row => row[0]);
}

function predictOLS(coefs: number[], x: number[]): number {
    return coefs[0] + x.reduce((sum, xi, i) => sum + xi * coefs[i + 1], 0);
}

// Leave-one-out mean absolute error of an OLS fit on (X, y).
function looMAE(X: number[][], y: number[]): number {
    const n = X.length;
    let total = 0;
    for (let i = 0; i < n; i++) {
        const trainX = X.filter((_, j) => j !== i);
        const trainY = y.filter((_, j) => j !== i);
        const coefs = fitOLS(trainX, trainY);
        total += Math.abs(predictOLS(coefs, X[i]) - y[i]);
    }
    return total / n;
}

function mode(values: number[]): number {
    const counts = new Map<number, number>();
    for (const v of values) counts.set(v, (counts.get(v) ?? 0) + 1);
    let best = values[0], bestCount = -1;
    for (const [v, c] of counts) if (c > bestCount) { best = v; bestCount = c; }
    return best;
}

/**
 * Picks predictor columns (indices into `parties`) for one target party.
 * Baseline is the party's own Bund value. A second predictor (the strongest
 * cross-party effect) is added only if it beats the baseline under nested
 * leave-one-out cross-validation — partner selection happens on the 8
 * training points, never touching the held-out point. See CLAUDE.md 3.3.
 */
function selectPredictors(targetIdx: number, X: number[][], Y: number[][], numParties: number): { predictors: number[]; error: number } {
    const n = X.length;
    const yTarget = Y.map(row => row[targetIdx]);
    const ownOnlyError = looMAE(X.map(row => [row[targetIdx]]), yTarget);

    const chosenPartners: number[] = [];
    const outerErrors: number[] = [];
    for (let i = 0; i < n; i++) {
        const trainIdx = [...Array(n).keys()].filter(j => j !== i);
        const trainX = trainIdx.map(j => X[j]);
        const trainY = trainIdx.map(j => Y[j][targetIdx]);

        let bestPartner = -1;
        let bestError = Infinity;
        for (let p = 0; p < numParties; p++) {
            if (p === targetIdx) continue;
            const error = looMAE(trainX.map(row => [row[targetIdx], row[p]]), trainY);
            if (error < bestError) { bestError = error; bestPartner = p; }
        }
        chosenPartners.push(bestPartner);

        const coefs = fitOLS(trainX.map(row => [row[targetIdx], row[bestPartner]]), trainY);
        const prediction = predictOLS(coefs, [X[i][targetIdx], X[i][bestPartner]]);
        outerErrors.push(Math.abs(prediction - Y[i][targetIdx]));
    }
    const crossError = outerErrors.reduce((a, b) => a + b, 0) / n;

    return crossError < ownOnlyError
        ? { predictors: [targetIdx, mode(chosenPartners)], error: crossError }
        : { predictors: [targetIdx], error: ownOnlyError };
}

export function predictResults(regionData: ElectionResult[], bundData: ElectionResult[], predictionData: ElectionResult): PredictionResult[] {
    const parties = RELEVANT_PARTIES;

    const bundByName = new Map(bundData.map(row => [row.name, row]));
    const paired = regionData.map(region => {
        const bund = bundByName.get(region.name);
        if (!bund) {
            throw new Error(`Keine Bundesergebnisse für Wahl "${region.name}" gefunden`);
        }
        return { region, bund };
    });

    const X = paired.map(({ bund }) => parties.map(p => (bund as any)[p] ?? 0));
    const Y = paired.map(({ region }) => parties.map(p => (region as any)[p] ?? 0));
    const x_pred = parties.map(p => (predictionData as any)[p] ?? 0);

    const fits = parties.map((_, targetIdx) => {
        const { predictors, error } = selectPredictors(targetIdx, X, Y, parties.length);
        const yTarget = Y.map(row => row[targetIdx]);
        const coefs = fitOLS(X.map(row => predictors.map(p => row[p])), yTarget);
        const value = predictOLS(coefs, predictors.map(p => x_pred[p]));
        return { value, error };
    });

    const sum = fits.reduce((acc, fit) => acc + fit.value, 0);
    // Sonstige is the residual 1 − Σ(6 Parteien); it has no OLS fit of its own, 
    const sonstige = Math.max(0, 1 - sum);
    const total = sum + sonstige;

    return [
        ...parties.map((name, index) => ({ name, value: fits[index].value / total, error: fits[index].error })),
        { name: "Sonstige", value: sonstige / total },
    ];
}

export function toResult(electionResult: ElectionResult): Result[] {
    return [{ name: "CDU", value: electionResult.CDU },
    { name: "SPD", value: electionResult.SPD },
    { name: "GRÜNE", value: electionResult.GRÜNE },
    { name: "LINKE", value: electionResult.LINKE },
    { name: "AfD", value: electionResult.AfD },
    { name: "FDP", value: electionResult.FDP },
    { name: "Sonstige", value: 1 - (electionResult.CDU + electionResult.SPD + electionResult.GRÜNE + electionResult.LINKE + electionResult.FDP + electionResult.AfD) },]
}