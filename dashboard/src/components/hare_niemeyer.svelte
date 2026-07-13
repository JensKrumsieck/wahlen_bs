<script lang="ts">
    import { Jumper } from "svelte-loading-spinners";
    import { getResultsForRegion, getSurveyData, predictResults, results_bund, toResult } from "$lib/elections";
    import type { Election, ElectionResult, Result } from "$lib/types";
    import { onMount } from "svelte";
    import { AxisX, AxisY, BarY, Plot } from "svelteplot";
    import { colors, RELEVANT_PARTIES } from "$lib/config";

    export let elections: Election[];

    const beznum = [111, 112, 120, 130, 211, 212, 221, 222, 310, 321, 322, 330];
    let surveyData: ElectionResult;
    let prediction: Record<string, Result[]> = Object.fromEntries(beznum.map((num) => [num.toString(), []]));
    let loading = true;

    const SEATS = 56;
    let fraction_seats: Result[];

    let selectedParty: keyof ElectionResult = "GRÜNE";
    const gemeindewahlbereiche: Record<string, number[]> = {
        "11": [111, 112],
        "12": [120],
        "13": [130],
        "21": [211, 212],
        "22": [221, 222],
        "31": [310],
        "32": [321, 322],
        "33": [330],
    };
    let turnout: Record<string, number> = {
        "11": 62.2,
        "12": 65.3,
        "13": 51.9,
        "21": 59.5,
        "22": 43.1,
        "31": 48.9,
        "32": 59.7,
        "33": 53.5,
    };
    let eligble: any = { "111": 8833, "112": 16419, "120": 21619, "130": 21140, "211": 16323, "212": 10977, "221": 17009, "222": 10038, "310": 26974, "321": 17398, "322": 9864, "330": 21134 };

    onMount(async () => {
        surveyData = await getSurveyData();

        beznum.forEach((num) => {
            let regionData = getResultsForRegion(num, elections);
            let region_prediction = predictResults(regionData, results_bund, surveyData);
            prediction[num.toString()] = region_prediction;
        });
        prediction = prediction;

        let regionData = getResultsForRegion(0, elections);
        let bs_prediction = predictResults(regionData, results_bund, surveyData);

        //calculate seats in parl
        fraction_seats = bs_prediction.map((item) => ({
            ...item,
            value: Math.round(item.value * SEATS),
        }));
        let sum = fraction_seats.slice(0, -1).reduce((acc, item) => acc + item.value, 0);
        fraction_seats[fraction_seats.length - 1] = {
            ...fraction_seats[fraction_seats.length - 1],
            value: SEATS - sum,
        };

        loading = false;
    });

    $: gwbResults = loading
        ? {}
        : Object.fromEntries(
              Object.entries(gemeindewahlbereiche).map(([gwbKey, bezirke]) => {
                  let totalWaehler = 0;
                  let partyWaehler = 0;

                  bezirke.forEach((bez) => {
                      const wahlberechtigte = eligble[bez];
                      const waehler = wahlberechtigte * (turnout[gwbKey] / 100);
                      const partyShare = prediction[bez.toString()]?.find((r) => r.name === selectedParty)?.value ?? 0;

                      totalWaehler += waehler;
                      partyWaehler += waehler * partyShare;
                  });

                  return [
                      gwbKey,
                      {
                          waehler: Math.round(totalWaehler),
                          partyWaehler: Math.round(partyWaehler),
                      },
                  ];
              }),
          );

    $: hareNiemeyerResults = (() => {
        if (loading || !fraction_seats) return [];

        const totalSeatsForParty = fraction_seats.find((r) => r.name === selectedParty)?.value ?? 0;
        const entries = Object.entries(gwbResults) as [string, (typeof gwbResults)[string]][];
        const sumPartyWaehler = entries.reduce((acc, [, r]) => acc + r.partyWaehler, 0);

        const withRaw = entries.map(([key, r]) => {
            const anteil = sumPartyWaehler > 0 ? r.partyWaehler / sumPartyWaehler : 0;
            const raw = anteil * totalSeatsForParty;
            const floor = Math.floor(raw);
            return { key, ...r, anteil, raw, floor, remainder: raw - floor };
        });

        const remainingSeats = totalSeatsForParty - withRaw.reduce((acc, r) => acc + r.floor, 0);

        const bonusKeys = new Set(
            [...withRaw]
                .sort((a, b) => b.remainder - a.remainder)
                .slice(0, remainingSeats)
                .map((r) => r.key),
        );

        return withRaw
            .map((r) => ({
                ...r,
                bonus: bonusKeys.has(r.key) ? 1 : 0,
                sitze: r.floor + (bonusKeys.has(r.key) ? 1 : 0),
            }))
            .sort((a, b) => a.key.localeCompare(b.key));
    })();
</script>

{#if loading}
    <Jumper />
{:else}
    <Plot x={{ type: "band" }} y={{ interval: 1, percent: false }}>
        <AxisX title="" />
        <AxisY title="" />
        <BarY data={fraction_seats} x="name" y="value" sort={{ channel: "name" }} fill={(c: { name: string }) => colors[c.name]} />
    </Plot>

    <h2>Vorhersage der Zusammensetzung</h2>
    <select name="party_select" title="Bitte Partei auswählen" bind:value={selectedParty} class="form-select p-2.5 text-sm rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800">
        <option value={-1} disabled>Bitte Partei auswählen</option>
        {#each RELEVANT_PARTIES as party}
            <option value={party}>{party}</option>
        {/each}
    </select>

    <h3>Sitzverteilung {selectedParty} nach Hare-Niemeyer</h3>
    <table class="w-full text-sm text-left">
        <thead>
            <tr>
                <th>GWB</th>
                <th>Wähler</th>
                <th>{selectedParty} %</th>
                <th>{selectedParty}</th>
                <th>Anteil</th>
                <th>Sitze (roh)</th>
                <th>Sitze</th>
                <th>Rest</th>
                <th>Bonus</th>
                <th>GWB Sitze</th>
            </tr>
        </thead>
        <tbody>
            {#each hareNiemeyerResults as r}
                <tr>
                    <td>{r.key}</td>
                    <td>{r.waehler}</td>
                    <td>{(r.partyWaehler / r.waehler * 100).toFixed(2)}%</td>
                    <td>{r.partyWaehler}</td>
                    <td>{(r.anteil * 100).toFixed(2)}%</td>
                    <td>{r.raw.toFixed(4)}</td>
                    <td>{r.floor}</td>
                    <td>{r.remainder.toFixed(4)}</td>
                    <td>{r.bonus}</td>
                    <td><strong>{r.sitze}</strong></td>
                </tr>
            {/each}
        </tbody>
    </table>

    <p>Die Vorhersage erfolgt basierend der Bezirkvorhersagen, der Zahl der Wahlberechtigten von 2021 und der Wahlbeteiligung der letzten Kommunalwahl. Die Anzahl der Sitze wird nach dem Hare-Niemeyer-Verfahren berechnet.</p>

    <h3>Wahlbeteiligung nach Gemeindewahlbereich</h3>
    <div class="grid grid-cols-2 gap-4">
        {#each Object.keys(gemeindewahlbereiche) as gwbKey}
            <label class="flex flex-col text-sm">
                Gemeindewahlbereich {gwbKey}
                <input type="number" min="0" max="100" step="0.1" bind:value={turnout[gwbKey]} class="p-2 rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-800" />
            </label>
        {/each}
    </div>
{/if}
