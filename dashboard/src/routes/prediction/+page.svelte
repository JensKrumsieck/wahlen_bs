<script>
    import HareNiemeyer from "../../components/hare_niemeyer.svelte";
    export let data;
    const { elections } = data;
</script>

<div class="prose prose-sm sm:prose-base lg:prose-lg xl:prose-xl dark:prose-invert mx-auto">
    <h1>Zusammensetzung des Rates</h1>
    <HareNiemeyer {elections} />
</div>
