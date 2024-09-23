<script lang="ts">
    import { onMount } from 'svelte';
    import LoadState from '$lib/components/LoadState.svelte';
    import SaveState from '$lib/components/SaveState.svelte';
    import ScrollableList from '$lib/components/ScrollableList.svelte';
    import PairSelector from '$lib/components/PairSelector.svelte';
    import GiverURLs from '$lib/components/GiverURLs.svelte';
    //import { initializeWasm } from '$lib/stores/NameList';

    //onMount(async () => {
    //  initializeWasm();
    //});


</script>
<LoadState />
<ScrollableList />
<PairSelector />
<GiverURLs />
<SaveState />
