import { API_URL, ELECTION_MAP, RELEVANT_PARTIES } from "./config";
import MLR from 'ml-regression-multivariate-linear';
import { Polls, Query, Order, DataType } from 'german-election-polls';
// @ts-ignore
import results_csv from '$lib/data/results_germany.csv';
import type { Election, ElectionResult, Result } from "./types";


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

async function getDawumData(limit: Date) {
    const polls = new Polls();
    await polls.update();
    const query = polls.select([
        Query.include([DataType.Surveys]),
        Query.Survey.Release.isGreater(limit),
        Query.Survey.Parliament.Shortcut.is(['Bundestag']),
        Query.Survey.Sort.byParticipants(Order.Asc),
        Query.Survey.Sort.allResults(Order.Desc),
    ]);
    return await query;
}

export async function getSurveyData() {
    let now = Date.now();
    let limit = new Date(now - 1000 * 60 * 60 * 24 * 14);
    let dawum = await getDawumData(limit);

    let results: ElectionResult[] = [];
    for (const survey of dawum.surveys) {
        results.push({
            name: survey.institute.name,
            CDU: survey.results.find(r => r.party.shortcut === 'CDU/CSU')?.result ?? 0,
            SPD: survey.results.find(r => r.party.shortcut === 'SPD')?.result ?? 0,
            GRÜNE: survey.results.find(r => r.party.shortcut === 'Grüne')?.result ?? 0,
            LINKE: survey.results.find(r => r.party.shortcut === 'Linke')?.result ?? 0,
            FDP: survey.results.find(r => r.party.shortcut === 'FDP')?.result ?? 0,
            AfD: survey.results.find(r => r.party.shortcut === 'AfD')?.result ?? 0,
            Sonstige: survey.results.find(r => r.party.shortcut === 'Sonstige')?.result ?? 0,
        });
    }

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

    for (const result of results) {
        mean.CDU += result.CDU;
        mean.SPD += result.SPD;
        mean.GRÜNE += result.GRÜNE;
        mean.LINKE += result.LINKE;
        mean.FDP += result.FDP;
        mean.AfD += result.AfD;
        mean.Sonstige += result.Sonstige;
    }

    mean.CDU /= results.length * 100;
    mean.SPD /= results.length * 100;
    mean.GRÜNE /= results.length * 100;
    mean.LINKE /= results.length * 100;
    mean.FDP /= results.length * 100;
    mean.AfD /= results.length * 100;
    mean.Sonstige /= results.length * 100;

    return mean;
}

export function predictResults(regionData: ElectionResult[], bundData: ElectionResult[], predictionData: ElectionResult) {
    let X = bundData.map(row => [
        row.CDU ?? 0,
        row.SPD ?? 0,
        row.GRÜNE ?? 0,
        row.LINKE ?? 0,
        row.FDP ?? 0,
        row.AfD ?? 0,
    ]);

    let Y = regionData.map(row => [
        row.CDU ?? 0,
        row.SPD ?? 0,
        row.GRÜNE ?? 0,
        row.LINKE ?? 0,
        row.FDP ?? 0,
        row.AfD ?? 0,
    ]);

    X.push([0, 0, 0, 0, 0, 0]);
    Y.push([0, 0, 0, 0, 0, 0]);

    let x_pred = [
        predictionData.CDU ?? 0,
        predictionData.SPD ?? 0,
        predictionData.GRÜNE ?? 0,
        predictionData.LINKE ?? 0,
        predictionData.FDP ?? 0,
        predictionData.AfD ?? 0,
    ];

    let party = ["CDU", "SPD", "GRÜNE", "LINKE", "FDP", "AfD", "Sonstige"];

    const prediction = predictMLR(X, Y, x_pred);
    let sum = prediction.reduce((acc, val) => acc + val, 0);
    prediction.push(1 - sum);

    return prediction.map((value, index) => {
        return { name: party[index], value: value };
    });
}

function predictMLR(X: number[][], Y: number[][], X_pred: number[]): number[] {
    const mlr = new MLR(X, Y);
    const prediction = mlr.predict(X_pred);

    return prediction;
}

export function toResult(electionResult: ElectionResult): Result[] {
    return [{ name: "CDU", value: electionResult.CDU },
    { name: "SPD", value: electionResult.SPD },
    { name: "GRÜNE", value: electionResult.GRÜNE },
    { name: "LINKE", value: electionResult.LINKE },
    { name: "FDP", value: electionResult.FDP },
    { name: "AfD", value: electionResult.AfD },
    { name: "Sonstige", value: 1 - (electionResult.CDU + electionResult.SPD + electionResult.GRÜNE + electionResult.LINKE + electionResult.FDP + electionResult.AfD) },]
}