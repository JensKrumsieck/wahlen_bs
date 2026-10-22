<script lang="ts">
    import Timeline from "../../components/timeline.svelte";
    import { SITE_URL } from "#lib/config.js";
    export let data;
    const { elections, regions } = data;

    const title = "Zeitlicher Verlauf der Wahlergebnisse | Wahlanalyse aus Braunschweig";
    const description = "Zeitlicher Verlauf der Wahlergebnisse vergangener Wahlen in Braunschweig.";
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="description" content={description} />
    <link rel="canonical" href={SITE_URL + "/timeline"} />
    <meta property="og:type" content="website" />
    <meta property="og:title" content={title} />
    <meta property="og:description" content={description} />
    <meta property="og:url" content={SITE_URL + "/timeline"} />
    <meta name="twitter:card" content="summary" />
</svelte:head>

<div class="prose prose-sm sm:prose-base lg:prose-lg xl:prose-xl dark:prose-invert mx-auto">
    <h1 class="text-lg font-bold tracking-tight">Zeitlicher Verlauf der Wahlergebnisse</h1>
    <p>In dieser Ansicht können die Ergebnisse der vergangenen Wahlen in Braunschweig im zeitlichen Verlauf betrachtet werden.</p>
    <Timeline {elections} {regions} />
    <p>BTW: Bundestagswahl, LTW: Landtagswahl, KoW: Kommunalwahlen, EUW: Europawahl</p>
</div>
