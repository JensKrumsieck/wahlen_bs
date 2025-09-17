<script lang="ts">
    import { getResultsForRegion } from "$lib/elections";
    import { geoMercator } from "d3-geo";
    import { type Election, type ElectionResult } from "$lib/types";
    import { Geo, Plot } from "svelteplot";
    import geoJson from "$lib/assets/districts.geojson?raw";
    import { ELECTION_MAP, RELEVANT_PARTIES } from "$lib/config";
    import { Jumper } from "svelte-loading-spinners";
    import { onMount } from "svelte";

    let geoJson_data = JSON.parse(geoJson);
    let selectedElectionId = 1;
    let selectedParty: keyof ElectionResult = "GRÜNE";
    let colorMap: Record<string, string> = { CDU: "greys", SPD: "reds", GRÜNE: "greens", LINKE: "purples", FDP: "oranges", AfD: "blues" };

    let width = 600;
    let height = 600;
    $: projection = geoMercator().fitSize([width, height], geoJson_data);

    type FeatureWithProperties = {
        properties: any;
        [key: string]: any;
    };

    let loaded = false;
    onMount(async () => {
        loaded = true;
    });

    export let elections: Election[];
    export let regions: any[];

    $: selectedElection = elections.find((e) => e.id == selectedElectionId);
    $: regionsData = regions.reduce((dict, r) => {
        dict[r.id] = getResultsForRegion(r.id, elections).find((e) => e.name == `${ELECTION_MAP[(selectedElection?.name as keyof typeof ELECTION_MAP) || "Bundestagswahl"]}${selectedElection?.date}`);
        return dict;
    }, {}) as Record<number, ElectionResult>;
    $: mappedRegions = geoJson_data.features.map((feature: FeatureWithProperties) => ({
        ...feature,
        properties: {
            ...feature.properties,
            dataValue: Number(regionsData[feature.properties.BEZNUM]?.[selectedParty] ?? 0) * 100,
        },
    })) as FeatureWithProperties[];
</script>

<select name="election_select" title="Bitte Wahl auswählen" bind:value={selectedElectionId} class="form-select p-2.5 text-sm rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800">
    <option value={-1} disabled>Bitte Wahl auswählen</option>
    {#each elections.reverse() as election}
        <option value={election.id}>{election.name} {election.date}</option>
    {/each}
</select>

<select name="party_select" title="Bitte Partei auswählen" bind:value={selectedParty} class="form-select p-2.5 text-sm rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800">
    <option value={-1} disabled>Bitte Partei auswählen</option>
    {#each RELEVANT_PARTIES as party}
        <option value={party}>{party}</option>
    {/each}
</select>

{#if !loaded}
    <Jumper />
{:else}
    <div bind:clientWidth={width}>
        <Plot
            {height}
            projection={{ type: () => projection }}
            color={{
                scheme: colorMap[selectedParty],
                legend: true,
                label: `${selectedParty} (%)`,
                n: 10,
                type: "log",
            }}
        >
            <Geo data={mappedRegions} fill={(d) => d.properties.dataValue} stroke="black" strokeWidth={1} title={(d) => `${d.properties.BEZNAM}: ${d.properties.dataValue.toFixed(2)} %`} />
        </Plot>
    </div>
    <div class="mt-5">
        Wahlergebnis für <span class="font-bold">{selectedParty}</span> ({selectedElection?.name}
        {selectedElection?.date})
        <table>
            <tbody>
                {#each mappedRegions as r}
                    <tr>
                        <td>{r.properties.BEZNAM}</td>
                        <td>{r.properties.dataValue.toFixed(2)} %</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
