<script lang="ts">
    import Prediction from '../../components/prediction.svelte';

    export let data;
    const { elections, regions} = data;
</script>

<Prediction {elections} {regions} />