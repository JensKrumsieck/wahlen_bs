<script lang="ts">
    import Map from "../../components/map.svelte";
    export let data;
    const { elections, regions } = data;
</script>

<div class="prose prose-sm sm:prose-base lg:prose-lg xl:prose-xl dark:prose-invert mx-auto">
    <h2 class="text-lg font-bold tracking-tight">Wahlergebnisse nach Stadtbezirk</h2>
    <p>Die Karte zeigt die Ergebnisse der ausgewählten Wahl in den 12 Stadtbezirken der Stadt Braunschweig. Die vor 2021 existierenden 19 Stadtbezirke wurden auf gemäß ihrer Zusammenlegung auf die 12 Stadtbezirke umgerechnet.</p>
    <p>Die Karte wurde aus dem <a href="https://www.braunschweig.de/leben/stadtplanung_bauen/geoinformationen/ogd_stadtbezirke.php" target="_blank">Shape Dateien </a> der Stadt Braunschweig erstellt.</p>
    <Map {elections} {regions} />
</div>
