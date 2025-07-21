<script lang="ts">
    import { Jumper } from "svelte-loading-spinners";
    import { getResultsForRegion, getSurveyData, predictResults, results_bund, type ElectionResult } from "$lib/elections";
    import { AxisX, AxisY, BarY, Plot } from "svelteplot";
    import { colors } from "$lib/config";
    import { onMount } from "svelte";

    export let elections;
    export let regions;

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
    $: if (predictionData) {
        prediction = predictResults(regionData, results_bund, predictionData);
    }
</script>

<select bind:value={selectedRegion} class="form-select">
    <option value="" disabled>Select a region</option>
    <option value={0}>Stadt Braunschweig</option>
    {#each regions as region}
        <option value={region.id}>{region.id} - {region.name}</option>
    {/each}
</select>

{#if !prediction || !trend}
    <Jumper />
{:else}
    <Plot x={{ type: "band" }} y={{ percent: true }}>
        <AxisX title="" />
        <AxisY title="" />
        <BarY inset={8} data={prediction} x="name" y="value" sort={{ channel: "name" }} fill={(c: { name: string }) => colors[c.name]} />
        <BarY data={trend} x="name" y="value" opacity={0.25} fill={(c: { name: string }) => colors[c.name]} />
    </Plot>
{/if}
