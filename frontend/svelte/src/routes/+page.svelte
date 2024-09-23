<script lang="ts">
    import { onMount } from 'svelte';
    import DefaultPage from './DefaultPage.svelte';
    import MessagePage from './MessagePage.svelte';

    let msg = null;

    onMount(() => {
      const params = new URLSearchParams(window.location.search);
      msg = params.get('msg');
    });
</script>

{#if msg}
  <MessagePage message={msg} />
{:else}
  <DefaultPage />
{/if}
