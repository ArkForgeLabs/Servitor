<script lang="ts">
  import type Editor from "$lib/node_graph";
  import { editor_store } from "$lib/store";

  let editor: Editor;
  editor_store.subscribe(async (value) => {
    if (value) {
      editor = value;
    }
  });
</script>

<div id="top_bar">
  <h3>Node Graph</h3>

  <button
    on:click={() => {
      const JSON_FORM = editor.toJSON();
      console.log(JSON_FORM);

      fetch("/apiv1/graph/create", {
        method: "POST",
        body: JSON.stringify(JSON_FORM),
      });
    }}>test</button
  >
</div>

<style>
  #top_bar {
    padding: 20px;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    height: 50px;
    width: 100vw;
    background: var(--darkreader-bg--color-surface-200);
  }
</style>
