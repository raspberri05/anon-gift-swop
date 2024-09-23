<script>
  import { onMount } from 'svelte';
  import { names } from '$lib/stores/NameList';
  import { selection } from '$lib/stores/Selection';
  import { pairs } from '$lib/stores/ForbiddenPairs';
  import { history } from '$lib/stores/AssignmentHistory';

  function decodeBase64Json(base64) {
    try {
      const jsonString = atob(base64);
      return JSON.parse(jsonString);
    } catch (e) {
      console.error("Failed to decode or parse the base64 string:", e);
      return null;
    }
  }

  function updateStoresFromParams(params) {
    if (params.names) {
      names.update(() => params.names);
    }
    if (params.selection) {
      selection.update(() => new Set(params.selection));
    }
    if (params.pairs) {
      pairs.update(() => params.pairs);
    }
    if (params.history) {
      history.update(() => params.history);
    }
  }

  onMount(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const startWithParam = urlParams.get('startwith');
    
    if (startWithParam) {
      const decodedData = decodeBase64Json(startWithParam);

      if (decodedData) {
        updateStoresFromParams(decodedData);
      }
    }
  });
</script>
