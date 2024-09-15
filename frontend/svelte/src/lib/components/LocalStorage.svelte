<script lang="ts">
    import { onMount } from 'svelte';

    export let getValue: () => string;
    export let setValue: (value: string) => void;
    export let keyPrefix: string;

    let saveName: string = "";
    let selectedItem: string = "";

    let savedItems: { name: string }[] = [];

    function loadItems(): void {
        savedItems = [];
        for (let i = 0; i < localStorage.length; i++) {
            const key = localStorage.key(i);
            if (key && key.startsWith(keyPrefix)) {
                savedItems.push({ name: key.slice(keyPrefix.length) });
            }
        }
        
        if (!savedItems.length) {
            selectedItem = "";
        }

    }

    function saveItem(): void {
        if (saveName.trim() === "") {
            alert("Name cannot be empty!");
            return;
        }

        const valueToSave = getValue();
        const prefixedKey = keyPrefix + saveName;
        if (localStorage.getItem(prefixedKey)) {
            if (!confirm(`An item with the name "${saveName}" already exists. Do you want to overwrite it?`)) {
                return;
            }
        }

        localStorage.setItem(prefixedKey, valueToSave);
        selectedItem = saveName;
        loadItems();
    }

    function loadItem(): void {
        const prefixedKey = keyPrefix + selectedItem;
        const value = localStorage.getItem(prefixedKey);
        if (value) {
            setValue(value);
            saveName = selectedItem;
        }
    }

    function deleteItem(): void {
        if (selectedItem && confirm(`Are you sure you want to delete the item "${selectedItem}"?`)) {
            const prefixedKey = keyPrefix + selectedItem;
            localStorage.removeItem(prefixedKey);
            loadItems();
            selectedItem = "";
        }
    }

    onMount(() => {
      loadItems();
    });
</script>

<style>
    .row {
        display: flex;
        align-items: center;
        gap: 10px;
    }

    button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
</style>

<div class="row">
    <input
        bind:value={saveName}
        placeholder="Enter save name"/>

    <button on:click={saveItem}>Save</button>

    <select bind:value={selectedItem}>
        <option value="" disabled selected>Saved entries</option>
        {#each savedItems as item}
            <option value={item.name}>{item.name}</option>
        {/each}
    </select>

    <button on:click={loadItem} disabled={selectedItem === ""}>Load</button>
    <button on:click={deleteItem} disabled={selectedItem === ""}>Delete</button>
</div>
