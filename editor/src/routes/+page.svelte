<script lang="ts">
  import NodeGraph from "$lib/components/node_graph.svelte";
  import NodeList from "$lib/components/node_list.svelte";
</script>

<div>
  <NodeList />
  <NodeGraph />
</div>

<style>
  div {
    display: flex;
  }
</style>
