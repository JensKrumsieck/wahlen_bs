<script lang="ts">
  import Prediction from "../components/prediction.svelte";

  export let data;
  const { elections, regions } = data;
</script>

<div
  class="prose prose-sm sm:prose-base lg:prose-lg xl:prose-xl dark:prose-invert mx-auto"
>
  <h2 class="text-lg font-bold tracking-tight">
    Vorhersage des Wahlergebnisses
  </h2>
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
