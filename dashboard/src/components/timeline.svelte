<script lang="ts">
    import { getResultsForRegion } from "$lib/elections";
    import { AxisX, AxisY, Line, Plot, Pointer, Text } from "svelteplot";
    import { colors, RELEVANT_PARTIES } from "$lib/config";
    import { type Election } from "$lib/types";
    import { Jumper } from "svelte-loading-spinners";
    import { onMount } from "svelte";

    export let elections: Election[];
    export let regions: any[];

    const parseDate = (c: any) => new Date(c["name"].substring(3));
    let selectedRegion = 0;

    let loaded = false;
    onMount(async () => {
        loaded = true;
    });

    $: regionData = getResultsForRegion(selectedRegion, elections);
</script>

<select name="region_select" title="Region für Timeline wählen" bind:value={selectedRegion} class="form-select p-2.5 text-sm rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800">
    <option value={-1} disabled>Region für Timeline wählen</option>
    <option value={0}>Stadt Braunschweig</option>
    {#each regions as region}
        <option value={region.id}>{region.id} - {region.name}</option>
    {/each}
</select>

<div class="mt-5 center">
    {#if !loaded}
        <Jumper />
    {:else}
        <Plot x={{ domain: [new Date(2007, 0, 1), new Date(2025, 0, 1)], interval: "2 years" }} y={{ percent: true, domain: [0, 0.45] }}>
            <AxisX title="" />
            <AxisY title="" />

            {#each RELEVANT_PARTIES as party}
                <Line data={regionData} x={parseDate} y={party} stroke={colors[party]} strokeWidth={3} text={party} marker="dot" curve="linear" />
                <Pointer data={regionData} x={parseDate} y={party}>
                    {#snippet children({ data })}
                        <Text {data} text={(c) => `${party}: ${((c as any)[party] * 100)?.toFixed(2)}% (${c.name})`} x={parseDate} y={party} dy={10} fontSize={15} />
                    {/snippet}
                </Pointer>
            {/each}
        </Plot>

        <div class="overflow-x-scroll">
            <table>
                <thead>
                    <tr>
                        <th scope="col"></th>
                        {#each RELEVANT_PARTIES as party}
                            <th scope="col">{party}</th>
                        {/each}
                    </tr>
                </thead>
                <tbody>
                    {#each regionData as election}
                        <tr>
                            <td>{election.name}</td>
                            {#each RELEVANT_PARTIES as party}
                                <td>
                                    {#if !isNaN((election as any)[party])}
                                        {((election as any)[party] * 100).toFixed(2)}%
                                    {:else}
                                        –
                                    {/if}
                                </td>
                            {/each}
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
</div>
