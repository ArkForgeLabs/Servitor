<script lang="ts">
  import { TableControl } from "$lib/node_graph";
  export let data: TableControl = new TableControl("", { key: "value" });
</script>

<table>
  <th>
    <span>{data.name}</span> <br />
  </th>
  <tbody>
    {#each Object.keys(data.value) as key}
      <tr>
        <td>
          <input value={key} />
        </td>
        <td>
          <input value={data.value[key]} />
        </td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
</style>
