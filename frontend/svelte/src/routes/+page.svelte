<script lang="ts">
    import { onMount } from 'svelte';
    import LocalStorage from '$lib/components/LocalStorage.svelte';
    import init, { greet } from '$lib/wasm/rust_wasm.js';

    let currentValue: string = "";

    function getValue() {
        return currentValue;
    }

    function setValue(value: string) {
        console.log(value)
        currentValue = value;
    }

    onMount(async () => {
        await init();
        greet("Svelte");
    });
</script>

<div>
    <label for="name">Value: </label>
    <input id="name" bind:value={currentValue} placeholder="Enter some text" />
</div>

<!-- Use the generic component -->
<LocalStorage {getValue} {setValue} keyPrefix="data-" />
