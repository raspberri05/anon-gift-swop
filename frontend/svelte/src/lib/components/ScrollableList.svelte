<script>
  import { onMount } from "svelte";
  import { names } from '$lib/stores/NameList';
  import { selection } from '$lib/stores/Selection';

  let name = "";

  function addName() {
    if (name.trim()) {
      let n = $names.length;
      names.update(arr => [...arr, name]);
      selection.update(set => new Set([...set, n]));
      name = ""; // clear input field
    }
  }

  function toggleSelection(index, checked) {
    selection.update(set => {
      const newSet = new Set(set);
      if (checked) {
        newSet.add(index);
      } else {
        newSet.delete(index);
      }
      return newSet;
    });
  }
</script>


<style>
  
  @import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;500&display=swap');

  * {
    margin: 0;
    padding: 0;
  }

  .container {
    margin:10px;
    font-family: 'Roboto', sans-serif;
    display: flex;
    flex-direction: column;
    width: 400px;
    padding: 20px;
    margin: 0 auto;
    border: 2px solid #ccc;
    border-radius: 8px;
    background-color: #f9f9f9;
    max-width: 100%; 
  }

  input[type="text"] {
    padding: 10px;
    font-size: 16px;
    border: 2px solid #ccc;
    border-radius: 4px;
    margin-bottom: 10px;
  }

  button {
    padding: 10px;
    font-size: 16px;
    background-color: #4caf50;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    width: 100%;
  }

  button:hover {
    background-color: #45a049;
  }

  .list {
    margin-top:10px;
    min-height: 100px;
    max-height: 200px;
    overflow-y: auto;
    border: 2px solid #ccc;
    border-radius: 4px;
    padding: 10px;
    background-color: white;
    overflow-x: hidden; 
    resize: vertical;
  }

  .list-item {
    display: flex;
    align-items: center;
    padding: 8px 0;
    border-bottom: 1px solid #ddd;
  }

  .list-item:last-child {
    border-bottom: none;
  }

  .list-item span {
    font-size: 18px;
    flex: 1; 
    color: #333;
    word-break: break-word;
  }

  .list-item input[type="checkbox"] {
    transform: scale(1.5);
    margin-right: 10px;
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    transform-origin: center;
    margin-left:10px;
  }
</style>

<div class="container">
  <input
    class="entry"
    type="text"
    bind:value={name}
    placeholder="Enter a name"
    on:keypress={(e) => e.key === 'Enter' && addName()}
  />
  <button on:click={addName}>Add Participant</button>

  <div class="list">
    {#each $names as name, index}
      <div class="list-item">
        <input type="checkbox" 
          checked={$selection.has(index)}
          on:change={(e) => toggleSelection(index, e.target.checked)}
        />
        <span>{name}</span>
      </div>
    {/each}
  </div>
</div>
