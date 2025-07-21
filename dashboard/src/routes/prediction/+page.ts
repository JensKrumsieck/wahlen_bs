import { get_available_elections, get_available_regions, type Election } from '$lib/elections';
import type { LoadEvent } from '@sveltejs/kit';
import { API_URL } from '$lib/config';

export async function load({ fetch }: LoadEvent) {
    const elections = await get_available_elections(fetch);

    let electionData: Election[] = [];
    for (const election of elections) {
        if (["Bundestagswahl", "Europawahl"].includes(election.name)) {
            const response = await fetch(API_URL + `/election/${election.id}?primary_vote=false`);
            electionData.push(await response.json());
        }
    }

    const regions = await get_available_regions(fetch);

    return { elections: electionData, regions };
}
