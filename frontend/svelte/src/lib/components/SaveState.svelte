<script>
  import { onMount } from 'svelte';
  import { derived, writable } from 'svelte/store';
  import { names } from '$lib/stores/NameList';
  import { selection } from '$lib/stores/Selection';
  import { pairs } from '$lib/stores/ForbiddenPairs';
  import { history } from '$lib/stores/AssignmentHistory';
  import {assignment} from '$lib/stores/CurrentAssignment';

  let href = writable("http://unknown");

  function encodeBase64Json(obj) {
    try {
      const jsonString = JSON.stringify(obj);
      return btoa(jsonString);
    } catch (e) {
      console.error("Failed to encode JSON to Base64:", e);
      return null;
    }
  }

  function updateEncodedUrl(assignment) {
    const data = {};

    if ($names) {
      data.names = $names;
    }
    if ($selection) {
      data.selection = Array.from($selection);
    }
    if ($pairs) {
      data.pairs = $pairs;
    }
    
    if (assignment){    	
      data.history = [assignment, ...$history];
    } else if ($history) {
      data.history = $history;
    }

    const encodedData = encodeBase64Json(data);
    
    if (encodedData) {
      const url = new URL($href);
      url.searchParams.set('startwith', encodedData);
      return url.toString();
    }
    return "";
  };

  onMount(() => {
    href.update(o => window.location.href);
  });

  let encodedUrl = derived([href, names, selection, pairs], () => updateEncodedUrl() );
  let encodedUrl2 = derived([href, assignment], () => $assignment ? updateEncodedUrl($assignment) : null );

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
   
  .link-button {
    display: inline-block;
    padding: 10px 15px;
    margin: 5px;
    text-decoration: none;
    color: white;
    border-radius: 5px;
    transition: background-color 0.3s;
    text-align: center;
  }

  .encode-button {
    background-color: #4CAF50; 
  }

  .encode-button:hover {
    background-color: #45a049; 
  }

  .assignment-button {
    background-color: #007bff; 
  }

  .assignment-button:hover {
    background-color: #0056b3; 
  }
</style>

<div class="container">
<a target="_blank" class="link-button encode-button" href={$encodedUrl}>
  Current setup
</a>
{#if $assignment}
<a target="_blank" class="link-button assignment-button" href={$encodedUrl2}>
  Current state (include latest assignment)
</a>
{/if}
</div>

