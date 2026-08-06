<script lang="ts">
  import { Jumper } from "svelte-loading-spinners";
  import {
    getResultsForRegion,
    getSurveyData,
    predictResults,
    results_bund,
    toResult,
  } from "$lib/elections";
  import { AxisX, AxisY, BarY, Plot, RuleX } from "svelteplot";
  import { colors, RELEVANT_PARTIES } from "$lib/config";
  import { onMount } from "svelte";
  import {
    type Election,
    type ElectionResult,
    type Result,
    type PredictionResult,
  } from "$lib/types";

  export let elections: Election[];
  export let regions: any[];

  function lighten(hex: string, amount = 0.5): string {
    const n = parseInt(hex.slice(1), 16);
    const r = Math.round(
      ((n >> 16) & 255) + (255 - ((n >> 16) & 255)) * amount,
    );
    const g = Math.round(((n >> 8) & 255) + (255 - ((n >> 8) & 255)) * amount);
    const b = Math.round((n & 255) + (255 - (n & 255)) * amount);
    return `#${((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1)}`;
  }

  let predictionData: ElectionResult;
  let lastElection: Result[];
  let trend: Result[];
  let prediction: PredictionResult[];
  let predictionError: string | null = null;

  onMount(async () => {
    try {
      predictionData = await getSurveyData();
      trend = toResult(predictionData);
    } catch (e) {
      predictionError = e instanceof Error ? e.message : String(e);
    }
  });
  let selectedRegion = 0;

  $: regionData = getResultsForRegion(selectedRegion, elections);
  $: if (predictionData && regionData) {
    try {
      lastElection = toResult(
        regionData.find((d) => d.name == "BTW2025") || regionData[0],
      );
      prediction = predictResults(regionData, results_bund, predictionData);
      predictionError = null;
    } catch (e) {
      predictionError = e instanceof Error ? e.message : String(e);
    }
  }
</script>

<select
  name="region_select"
  title="Region für Vorhersage wählen"
  bind:value={selectedRegion}
  class="form-select p-2.5 text-sm rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800"
>
  <option value={-1} disabled>Region für Vorhersage wählen</option>
  <option value={0}>Stadt Braunschweig</option>
  {#each regions as region}
    <option value={region.id}>{region.id} - {region.name}</option>
  {/each}
</select>
<div class="mt-5 center">
  {#if predictionError}
    <p class="text-red-600 dark:text-red-400">Vorhersage konnte nicht geladen werden: {predictionError}</p>
  {:else if !prediction || !trend}
    <Jumper />
  {:else}
    <Plot x={{ type: "band" }} y={{ percent: true }}>
      <AxisX title="" />
      <AxisY title="" />
      <BarY
        inset={8}
        data={prediction}
        x="name"
        y="value"
        sort={{ channel: "name" }}
        fill={(c: { name: string }) => colors[c.name]}
      />
      <BarY
        data={trend}
        x="name"
        y="value"
        opacity={0.25}
        fill={(c: { name: string }) => colors[c.name]}
      />
      <RuleX
        data={prediction.filter((p) => p.error != null)}
        x="name"
        y1={(p: PredictionResult) => p.value - (p.error ?? 0)}
        y2={(p: PredictionResult) => p.value + (p.error ?? 0)}
        strokeWidth={5}
        stroke={(c: { name: string }) => lighten(colors[c.name], .4)}
      />
    </Plot>
    <p class="text-xs text-gray-500 dark:text-gray-400">
      Die Fehlerbalken zeigen den mittleren absoluten Fehler je Partei unter
      Kreuzvalidierung (Leave-one-out). Für "Sonstige" gibt es keinen eigenen
      Fehlerwert, da der Anteil als Rest berechnet wird. Zu beachten: Das Modell
      ist auf Bundestags- und Europawahlen trainiert. Lokale Listen wie BIBS
      kann es grundsätzlich nicht abbilden.
    </p>
    <div
      class="overflow-x-auto rounded-lg border border-gray-200 dark:border-gray-700"
    >
      <table class="w-full min-w-max border-collapse text-sm">
        <thead>
          <tr class="bg-gray-50 dark:bg-gray-800">
            <th
              scope="col"
              class="!pl-3 px-3 py-2 text-left font-medium text-gray-500 dark:text-gray-400"
            ></th>
            {#each RELEVANT_PARTIES.concat("Sonstige") as name}
              <th
                scope="col"
                class="px-3 py-2 text-right font-semibold whitespace-nowrap last:!pr-3"
                style="box-shadow: inset 0 2px 0 0 {colors[name]}"
              >
                {name}
              </th>
            {/each}
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-100 dark:divide-gray-700/60">
          <tr class="text-gray-500 dark:text-gray-400">
            <td class="!pl-3 px-3 py-2 text-left whitespace-nowrap">
              Deutschlandtrend (Umfragen)
            </td>
            {#each trend as party}
              <td class="px-3 py-2 text-right tabular-nums last:!pr-3">
                {(party.value * 100).toFixed(1)}%
              </td>
            {/each}
          </tr>
          <tr class="bg-blue-50/60 font-semibold dark:bg-blue-900/20">
            <td class="!pl-3 px-3 py-2 text-left whitespace-nowrap">
              Vorhersage ({selectedRegion == 0
                ? "Stadt Braunschweig"
                : regions.find((e) => e.id == selectedRegion).name})
            </td>
            {#each prediction as party}
              <td class="px-3 py-2 text-right tabular-nums last:!pr-3">
                {(party.value * 100).toFixed(1)}%
              </td>
            {/each}
          </tr>
          <tr
            class="text-xs text-gray-400 italic dark:text-gray-500"
          >
            <td class="!pl-3 px-3 py-2 text-left whitespace-nowrap">
              ± Unsicherheit (LOOCV)
            </td>
            {#each prediction as party}
              <td class="px-3 py-2 text-right tabular-nums last:!pr-3">
                {party.error != null
                  ? "± " + (party.error * 100).toFixed(1) + "%"
                  : "–"}
              </td>
            {/each}
          </tr>
          <tr class="text-gray-500 dark:text-gray-400">
            <td class="!pl-3 px-3 py-2 text-left whitespace-nowrap">
              Bundestagswahl 2025 ({selectedRegion == 0
                ? "Stadt Braunschweig"
                : regions.find((e) => e.id == selectedRegion).name})
            </td>
            {#each lastElection as party}
              <td class="px-3 py-2 text-right tabular-nums last:!pr-3">
                {(party.value * 100).toFixed(1)}%
              </td>
            {/each}
          </tr>
        </tbody>
      </table>
    </div>
  {/if}
</div>
