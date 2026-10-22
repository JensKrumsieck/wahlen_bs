<script lang="ts">
    import Map from "../../components/map.svelte";
    import { SITE_URL } from "#lib/config.js";
    export let data;
    const { elections, regions } = data;

    const title = "Wahlergebnisse nach Stadtbezirk | Wahlanalyse aus Braunschweig";
    const description = "Kartenansicht der Wahlergebnisse in den 12 Stadtbezirken der Stadt Braunschweig.";
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="description" content={description} />
    <link rel="canonical" href={SITE_URL + "/map"} />
    <meta property="og:type" content="website" />
    <meta property="og:title" content={title} />
    <meta property="og:description" content={description} />
    <meta property="og:url" content={SITE_URL + "/map"} />
    <meta name="twitter:card" content="summary" />
</svelte:head>

<div class="prose prose-sm sm:prose-base lg:prose-lg xl:prose-xl dark:prose-invert mx-auto">
    <h1 class="text-lg font-bold tracking-tight">Wahlergebnisse nach Stadtbezirk</h1>
    <p>Die Karte zeigt die Ergebnisse der ausgewählten Wahl in den 12 Stadtbezirken der Stadt Braunschweig. Die vor 2021 existierenden 19 Stadtbezirke wurden auf gemäß ihrer Zusammenlegung auf die 12 Stadtbezirke umgerechnet.</p>
    <p>Die Karte wurde aus den <a href="https://www.braunschweig.de/leben/stadtplanung_bauen/geoinformationen/ogd_stadtbezirke.php" target="_blank">Shape Dateien</a> der Stadt Braunschweig erstellt.</p>
    <Map {elections} {regions} />
</div>
