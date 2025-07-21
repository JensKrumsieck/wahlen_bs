<script lang="ts">
	import "../app.css";

	let { children } = $props();
</script>

<svelte:head>
	<title>Wahlanalyse aus Braunschweig</title>
</svelte:head>

<header class="bg-[#00a646] p-2.5">
	<div class="container container-sm mx-auto flex justify-between items-center">
		<h1 class="font-black text-xl tracking-tight">Wahlanalyse</h1>
		<a href="https://github.com/JensKrumsieck/wahlen_bs" target="_blank" class="hover"><img src="github.svg" alt="GitHub" class="w-5" /></a>
	</div>
</header>
<div class="dark:bg-gray-800 min-h-screen px-2.5">
	<article class="pt-5">
		{@render children()}
	</article>
	<footer class="p-5 mt-12 text-center container container-md mx-auto dark:text-gray-400 text-gray-600 text-sm border-t dark:border-gray-600 border-gray-50">
		<div class="copy">
			Wahldaten wurden aus der <a class="underline" href="https://votemanager.kdo.de/03101000/index.html">Votemanager-Instanz der Stadt Braunschweig</a>, sowie dem
			<a class="underline" href="https://www3.braunschweig.de/statistik/2025_Wahl-Atlas/atlas.html">Wahlatlas</a> geharvestet. Ergebnisse, die auf den alten 19 Stadtbezirken basieren, wurden entsprechend der Zusammenlegung zu den 12
			Stadtbezirken umgerechnet. Die Daten der bundesweiten Ergebnisse wurden aus dem Portal der <a class="underline" href="https://www.bundeswahlleiterin.de/">Bundeswahlleiterin</a> entnommen.
		</div>
		<div class="mt-5 text-green-200">
			Ein Projekt von <a class="underline font-medium" href="https://jenskrumsieck.de">Dr. Jens Krumsieck</a>.
		</div>
	</footer>
</div>
