<script lang="ts">
    import Prediction from "../components/prediction.svelte";

    export let data;
    const { elections, regions } = data;
</script>

<div class="prose prose-sm sm:prose-base lg:prose-lg xl:prose-xl dark:prose-invert mx-auto">
    <h2 class="text-lg font-bold tracking-tight">Vorhersage des Wahlergebnisses</h2>
    <Prediction {elections} {regions} />
    <p>
        Die Vorhersage des Wahlergebnisses basiert auf einer Multivariaten Linearen Regression (MLR), bei der die Ergebnisse der entsprechenden Region mit den bundesweiten Ergebnissen in Relation gesetzt werden. Verwendet werden hier die Daten der
        Wahlen seit 2000 mit Ausnahme der Bundestagswahl 2005 (Die Daten fehlen im Portal der Stadt Braunschweig). Die Daten des Deutschlandtrends werden von <a href="https://dawum.de">DAWUM</a> bezogen. Hier wird ein ungewichteter Mittelwert der Umfragen
        der letzten 14 Tage verwendet, dieser ist im Hintergrund des Balkendiagramms dargestellt.
    </p>
</div>
