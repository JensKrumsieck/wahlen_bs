<script lang="ts">
    import { Jumper } from "svelte-loading-spinners";
    import { getResultsForRegion, getSurveyData, predictResults, results_bund, type Election, type ElectionResult } from "$lib/elections";
    import { AxisX, AxisY, BarY, Plot } from "svelteplot";
    import { colors } from "$lib/config";
    import { onMount } from "svelte";

    export let elections: Election[];
    export let regions: any[];

    type Result = {
        name: string;
        value: number;
    };

    let predictionData: ElectionResult;
    let trend: Result[];
    let prediction: Result[];

    onMount(async () => {
        predictionData = await getSurveyData();
        trend = [
            { name: "CDU", value: predictionData.CDU },
            { name: "SPD", value: predictionData.SPD },
            { name: "GRÜNE", value: predictionData.GRÜNE },
            { name: "LINKE", value: predictionData.LINKE },
            { name: "FDP", value: predictionData.FDP },
            { name: "AfD", value: predictionData.AfD },
            { name: "Sonstige", value: 1 - (predictionData.CDU + predictionData.SPD + predictionData.GRÜNE + predictionData.LINKE + predictionData.FDP + predictionData.AfD) },
        ];
    });
    let selectedRegion = 0;

    $: regionData = getResultsForRegion(selectedRegion, elections);
    $: if (predictionData && regionData) {
        prediction = predictResults(regionData, results_bund, predictionData);
    }
</script>

<select name="region_select" title="Region für Vorhersage wählen" bind:value={selectedRegion} class="form-select p-2.5 text-sm rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800">
    <option value={-1} disabled>Region für Vorhersage wählen</option>
    <option value={0}>Stadt Braunschweig</option>
    {#each regions as region}
        <option value={region.id}>{region.id} - {region.name}</option>
    {/each}
</select>
<div class="mt-5 center">
    {#if !prediction || !trend}
        <Jumper />
    {:else}
        <Plot x={{ type: "band" }} y={{ percent: true }}>
            <AxisX title="" />
            <AxisY title="" />
            <BarY inset={8} data={prediction} x="name" y="value" sort={{ channel: "name" }} fill={(c: { name: string }) => colors[c.name]} />
            <BarY data={trend} x="name" y="value" opacity={0.25} fill={(c: { name: string }) => colors[c.name]} />
        </Plot>
        <div class="overflow-x-scroll">
            <table>
                <thead>
                    <tr>
                        <th scope="col"></th>
                        <th scope="col">CDU</th>
                        <th scope="col">SPD</th>
                        <th scope="col">GRÜNE</th>
                        <th scope="col">LINKE</th>
                        <th scope="col">FDP</th>
                        <th scope="col">AfD</th>
                        <th scope="col">Sonstige</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td> Deutschlandtrend (Umfragen) </td>
                        {#each trend as party}
                            <td>
                                {(party.value * 100).toFixed(2)}%
                            </td>
                        {/each}
                    </tr>
                    <tr>
                        <td>
                            Vorhersage ({selectedRegion == 0 ? "Stadt Braunschweig" : regions.find((e) => e.id == selectedRegion).name})
                        </td>
                        {#each prediction as party}
                            <td>
                                {(party.value * 100).toFixed(2)}%
                            </td>
                        {/each}
                    </tr>
                </tbody>
            </table>
        </div>
    {/if}
</div>
