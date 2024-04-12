<script lang="ts">
  import Editor from "$lib/components/editor.svelte";
  import NodeList from "$lib/components/node_list.svelte";
</script>

<div>
  <NodeList />
  <Editor />
</div>

<style>
  div {
    display: flex;
  }
</style>
