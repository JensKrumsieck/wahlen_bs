<script lang="ts">
	import '../app.css';
	
	let { children } = $props();
</script>

<svelte:head>
	<title>Wahlergebnisse Braunschweig</title>
</svelte:head>

<div class="dark:bg-gray-800 min-h-screen">

{@render children()}
</div>