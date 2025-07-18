import { API_URL, ELECTION_MAP, RELEVANT_PARTIES } from "./config";
import MLR from 'ml-regression-multivariate-linear';
import * as tf from '@tensorflow/tfjs';
// @ts-ignore
import results_csv from '$lib/data/results_germany.csv';


export async function get_available_elections(fetch: typeof window.fetch) {
    const endpoint = `${API_URL}/election`;
    const response = await fetch(endpoint);
    if (!response.ok) {
        throw new Error(`Failed to fetch elections: ${response.statusText}`);
    }

    const elections = await response.json();
    return elections;
}

export async function get_available_regions(fetch: typeof window.fetch) {
    const endpoint = `${API_URL}/region`;
    const response = await fetch(endpoint);
    if (!response.ok) {
        throw new Error(`Failed to fetch regions: ${response.statusText}`);
    }

    const regions = await response.json();
    return regions;
}

export interface Election {
    id: number;
    name: string;
    date: number;
    region: Region[];
}

interface Region {
    id: number;
    name: string;
    turnout: Turnout[];
    votes: Vote[];
}

interface Turnout {
    eligible: number,
    primary_vote: boolean,
    voted: number,
    turnout: number
}
interface Vote {
    id: string,
    name: string,
    votes: number,
    primary_vote: boolean,
    percentage: number
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

export type ElectionResult = {
    name: string;
    CDU: number;
    SPD: number;
    GRÜNE: number;
    LINKE: number;
    FDP: number;
    AfD: number;
    Sonstige: number;
};

export async function predictResults(regionData: ElectionResult[], bundData: ElectionResult[], backend: "tf" | "mlr") {
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

    return backend == "mlr" ? predictMLR(X, Y) : await predictTF(X, Y);

}

function predictMLR(X: number[][], Y: number[][]): number[] {
    const mlr = new MLR(X, Y);
    const prediction = mlr.predict([.270, .153, .119, .100, .035, .227]);

    return prediction;
}

async function predictTF(X: number[][], Y: number[][]): Promise<number[]> {
    const tensorX = tf.tensor2d(X);
    const tensorY = tf.tensor2d(Y);

    const model = tf.sequential();
    model.add(tf.layers.dense({ units: 64, activation: 'relu', inputShape: [6] }));
    model.add(tf.layers.dense({ units: 32, activation: 'relu' }));
    model.add(tf.layers.dense({ units: 16, activation: 'relu' }));
    model.add(tf.layers.dense({ units: 6 }));
    model.compile({ optimizer: 'adam', loss: 'meanSquaredError' });
    await model.fit(tensorX, tensorY, {
        epochs: 500,
    });

    const prediction = model.predict(tf.tensor2d([[.270, .153, .119, .100, .035, .227]])) as tf.Tensor;
    const result = await prediction.data() as Float32Array;
    prediction.dispose();
    tensorX.dispose();
    tensorY.dispose();
    return Array.from(result);
}