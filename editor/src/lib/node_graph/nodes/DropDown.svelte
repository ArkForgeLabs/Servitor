<script lang="ts">
  import { DropDownControl } from "$lib/node_graph";
  export let data: DropDownControl = new DropDownControl("", []);
</script>

<div>
  <select name={data.name} bind:value={data.value}>
    {#each data.options as option, i}
      <option value={i}>{option}</option>
    {/each}
  </select>
</div>
