<script lang="ts">
  import Prediction from "../components/prediction.svelte";
  import { SITE_URL } from "$lib/config";

  export let data;
  const { elections, regions } = data;

  const title = "Wahlvorhersage Braunschweig | Wahlanalyse aus Braunschweig";
  const description =
    "Vorhersage des Wahlergebnisses in Braunschweig auf Basis linearer Regressionen zu bundesweiten Wahlergebnissen und aktuellen Umfragen.";
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />
  <link rel="canonical" href={SITE_URL + "/"} />
  <meta property="og:type" content="website" />
  <meta property="og:title" content={title} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={SITE_URL + "/"} />
  <meta name="twitter:card" content="summary" />
</svelte:head>

<div
  class="prose prose-xl sm:prose-base lg:prose-lg xl:prose-xl dark:prose-invert mx-auto"
>
  <h1 class="text-lg font-bold tracking-tight">
    Vorhersage des Wahlergebnisses
  </h1>
  <Prediction {elections} {regions} />
  <p>
    Die Vorhersage des Wahlergebnisses basiert auf je einer linearen Regression
    pro Partei, die das Braunschweiger Ergebnis mit dem bundesweiten Ergebnis
    derselben Partei in Relation setzt. Bei einzelnen Parteien wird zusätzlich
    ein zweiter bundesweiter Prädiktor herangezogen, aber nur, wenn er das
    Modell per Kreuzvalidierung nachweisbar verbessert. Diese Auswahl wird bei
    jeder neuen Wahl neu berechnet, nicht fest vorgegeben. Verwendet werden hier
    die Daten der neun Bundestags- und Europawahlen seit 2009. Die Daten des
    Deutschlandtrends werden von <a href="https://dawum.de">DAWUM</a> bezogen. Hier
    wird ein ungewichteter Mittelwert der Umfragen der letzten 14 Tage verwendet,
    dieser ist im Hintergrund des Balkendiagramms dargestellt.
  </p>
</div>
