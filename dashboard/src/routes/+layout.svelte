<script lang="ts">
	import "../app.css";

	let showMenu = $state(false);
	let { children } = $props();

	const navItems = [
		{ href: "/", label: "Wahlvorhersage" },
		{ href: "/timeline", label: "Zeitlicher Verlauf" },
	];
</script>

<svelte:head>
	<title>Wahlanalyse aus Braunschweig</title>
</svelte:head>

<header class="bg-[#00a646] p-2.5">
	<nav class="z-20">
		<div class="container container-sm mx-auto flex justify-between items-center">
			<a href="/" class="font-black text-xl tracking-tight">Wahlanalyse</a>
			<button class="md:hidden p-2" aria-label="Open menu" onclick={() => (showMenu = !showMenu)}>
				<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
				</svg>
			</button>
			<div class="hidden md:flex items-center space-x-4">
				{#each navItems as item}
					<a href={item.href} class="hover font-bold">{item.label}</a>
				{/each}
				<a href="https://github.com/JensKrumsieck/wahlen_bs" target="_blank" class="hover">
					<img src="github.svg" alt="GitHub" class="w-5" />
				</a>
			</div>
		</div>
		{#if showMenu}
			<div class="md:hidden bg-[#00a646] px-4 py-2 flex flex-col space-y-2">
				{#each navItems as item}
					<a href={item.href} class="hover flex items-center font-bold">
						{item.label}
					</a>
				{/each}
				<a href="https://github.com/JensKrumsieck/wahlen_bs" target="_blank" class="hover flex items-center">
					<img src="github.svg" alt="GitHub" class="w-5 mr-2" /> GitHub
				</a>
			</div>
		{/if}
	</nav>
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
