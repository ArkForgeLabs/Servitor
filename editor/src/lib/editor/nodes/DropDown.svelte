<script lang="ts">
  import { DropDownControl } from "$lib/editor";
  export let data: DropDownControl = new DropDownControl("", []);

  let selected = 0;
</script>

<div>
  <select name={data.name} bind:value={selected}>
    {#each data.options as option, i}
      <option value={i}>{option}</option>
    {/each}
  </select>
</div>
