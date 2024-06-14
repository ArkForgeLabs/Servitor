<script lang="ts">
  import TopBar from "$lib/components/top_bar.svelte";
  import NodeList from "$lib/components/node_list.svelte";
  import NodeGraph from "$lib/components/node_graph.svelte";
</script>

<TopBar />
<div>
  <NodeList />
  <NodeGraph />
</div>

<style>
  div {
    display: flex;
  }
</style>
