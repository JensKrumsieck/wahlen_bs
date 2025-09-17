<script lang="ts">
    import Timeline from "../../components/timeline.svelte";
    export let data;
    const { elections, regions } = data;
</script>

<div class="prose prose-sm sm:prose-base lg:prose-lg xl:prose-xl dark:prose-invert mx-auto">
    <h2 class="text-lg font-bold tracking-tight">Zeitlicher Verlauf der Wahlergebnisse</h2>
    <p>In dieser Ansicht können die Ergebnisse der vergangenen Wahlen in Braunschweig im zeitlichen Verlauf betrachtet werden. Kommunalwahlen werden derzeit nicht berücksichtigt.</p>
    <Timeline {elections} {regions} />
    <p>BTW: Bundestagswahl, LTW: Landtagswahl, KoW: Kommunalwahlen, EUW: Europawahl</p>
</div>
