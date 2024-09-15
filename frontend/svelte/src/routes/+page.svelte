<script lang="ts">
    import LocalStorage from '$lib/components/LocalStorage.svelte';

    let currentValue: string = "";

    function getValue() {
        return currentValue;
    }

    function setValue(value: string) {
        console.log(value)
        currentValue = value;
    }
</script>
<!--
<h2>Current Value</h2>
<p>{currentValue}</p>
-->

<div>
    <label for="name">Value: </label>
    <input id="name" bind:value={currentValue} placeholder="Enter some text" />
</div>

<!-- Use the generic component -->
<LocalStorage {getValue} {setValue} keyPrefix="data-" />
