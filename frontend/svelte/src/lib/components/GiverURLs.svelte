<script lang="ts">
  import { derived, writable } from 'svelte/store';
  import { onMount } from 'svelte';
  import { assignment } from '$lib/stores/CurrentAssignment';
  import { names } from '$lib/stores/NameList';
  import { selection } from '$lib/stores/Selection';
  import { pairs } from '$lib/stores/ForbiddenPairs';
  import { history } from '$lib/stores/AssignmentHistory';
  import init, { process_assignment } from '$lib/wasm/rust_wasm.js';
  

	const currentUrl = writable('');
  

  onMount(async () => {
  	await init();
    currentUrl.update(() => window.location.origin + window.location.pathname);
  });


  const assignment_names = derived(assignment, $assignment => {
    const result = [];

    if (Array.isArray($assignment)) {
      const length = $assignment.length;

      // To not expose the assignment in the ordering we need to randomise 
      // the display order
      const shuffledIndices = Array.from({ length }, (_, i) => i)
        .map(value => ({ value, sort: Math.random() }))
        .sort((a, b) => a.sort - b.sort)
        .map(({ value }) => value);
      

      for (let i = 0; i < length; i++) {
        const currentIndex = shuffledIndices[i];
        const nextIndex = (currentIndex + 1) % length;
        const giver = $names[$assignment[currentIndex]];
        const reciever = $names[$assignment[nextIndex]];
        result.push({
        giver, 
        reciever,
        url:createMessageUrl($currentUrl, giver, reciever),
        });
      }

    }
    return result;
  });

  function createMessageUrl(url: string, nameX: string, nameY: string): string {
    const message = `${nameX} you are giving to ${nameY}`;
    const encodedMessage = btoa(message);
    return `${url}?msg=${encodedMessage}`;
  }
  
  function createAssignment(){
	  assignment.update(() => {
      if ($selection && $selection.size > 0) {
        let res = process_assignment(Array.from($selection), $pairs, $history);
        if (res) return Array.from(res);
      }
      return null;

    });
  }
  
</script>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;500&display=swap');
  
  .container {
    margin: 10px;
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
  
  .pair-list {
    list-style-type: none; 
    padding: 0;           
    margin: 0;            
    max-width: 400px;     
    border: 1px solid #ccc; 
    border-radius: 8px;   
    background-color: #f9f9f9; 
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1); 
  }
  
  .pair-item {
    padding: 10px;        
    border-bottom: 1px solid #e0e0e0; 
    color: #333;          
    transition: background-color 0.3s; 
  }

  .pair-item:last-child {
    border-bottom: none; 
  }

  .pair-item:hover {
    background-color: #f1f1f1; 
  }

  .link-container {
    position: relative; 
    overflow: hidden;   
    max-width: 250px;   
    white-space: nowrap; 
    text-overflow: ellipsis; 
  }

  .link {
    color: #007BFF;      
    text-decoration: none; 
    font-weight: bold;    
    display: inline-block; 
    overflow: hidden;     
    max-width: 100%;      
  }

  .link-container::after {
    content: ""; 
    position: absolute;
    right: 0;
    width: 30px; 
    height: 100%;
    background: linear-gradient(to right, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 1) 100%);
  }

  .link:hover {
    text-decoration: underline; 
  }
  
  .styled-button {
    background-color: #007BFF; 
    color: white;              
    border: none;              
    border-radius: 5px;       
    padding: 10px 20px;       
    font-size: 16px;          
    cursor: pointer;           
    transition: background-color 0.3s, transform 0.2s; 
  }  
  .styled-button:hover {
    background-color: #0056b3; 
  }

  .styled-button:active {
    background-color: #004494; 
  }

  .styled-button:focus {
    outline: none;           
    box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.5); 
  }
</style>


<div class="container">
	<button class="styled-button" on:click={createAssignment}>
		Generate Assignment
	</button>
	{#if $assignment_names.length > 0}
  <ul class="pair-list">
    {#each $assignment_names as {giver, reciever, url}}
      <li class="pair-item">
        {giver} link is: 
        <span class="link-container">
          <a class="link" target="_blank" href={url}>
            {url}
          </a>
        </span>
      </li>
    {/each}
  </ul>
  {/if}
</div>

