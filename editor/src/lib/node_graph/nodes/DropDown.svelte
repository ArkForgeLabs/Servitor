<script lang="ts">
  import { DropDownControl } from "$lib/node_graph";
  export let data: DropDownControl = new DropDownControl("", []);
</script>

<div>
  <span>{data.name}</span> <br />
  <select name={data.name} bind:value={data.value}>
    {#each data.options as option, i}
      <option value={i}>{option}</option>
    {/each}
  </select>
</div>

<style>
  select {
    padding: 5px;
    outline: none;
  }

  select * {
    border: 1px solid white;
  }
</style>
