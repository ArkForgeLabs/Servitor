<script lang="ts">
  import { onMount } from "svelte";
  import Editor from "$lib/editor";

  let el: any;
  let editor: Editor;

  onMount(async () => {
    editor = new Editor(el);
  });
</script>

<div class="rete" bind:this={el} />

<style>
  .rete {
    position: relative;
    width: 100vw;
    height: 100vh;
    font-size: 1rem;
    text-align: left;
    border: 1px solid #cbd0db;

    background-size: 40px 40px;
    background-image: linear-gradient(to right, lightgrey 1px, transparent 1px),
      linear-gradient(to bottom, lightgrey 1px, transparent 1px);
  }

  :global(body) {
    margin: 0;
    overflow: hidden;
  }
</style>
