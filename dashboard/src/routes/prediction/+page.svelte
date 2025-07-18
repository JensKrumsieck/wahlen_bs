<script lang="ts">
    import { Jumper } from "svelte-loading-spinners";
    import { getResultsForRegion, predictResults, results_bund } from "$lib/elections";

    export let data;
    const { elections, regions, predictionData } = data;
    let selectedRegion = 0;
    let backend: "mlr" | "tf" = "mlr";

    $: regionData = getResultsForRegion(selectedRegion, elections);
    $: prediction = predictResults(regionData, results_bund, predictionData, backend);
</script>

<select bind:value={selectedRegion} class="form-select">
    <option value="" disabled>Select a region</option>
    <option value={0}>Stadt Braunschweig</option>
    {#each regions as region}
        <option value={region.id}>{region.id} - {region.name}</option>
    {/each}
</select>

<select bind:value={backend} class="form-select">
    <option value="mlr">Multivariate Lineare Regression</option>
    <option value="tf">Neuronales Netz</option>
</select>

{#await prediction}
    <Jumper size="60" />
{:then prediction}
    {#each prediction as value}
        {value},
    {/each}
{/await}
