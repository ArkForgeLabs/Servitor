<script lang="ts">
  import { TableControl } from "$lib/node_graph";
  import { IconTrash } from "$lib/icons/icons";
  export let data: TableControl = new TableControl("", { key: "value" });
  let keys = {};
  Object.keys(data.value).map((key) => {
    keys[key] = key;
  });

  let key_store = "";

  function on_key_new_input(key: string) {
    let keys_duplicates_length = Object.values(keys).filter(
      (found_key) => found_key == keys[key]
    ).length;

    let new_key = keys[key];
    if (new_key.toString().length > 0 && new_key != key) {
      if (keys_duplicates_length > 1) {
        new_key = `${new_key} (${keys_duplicates_length})`;
      }

      data.value[new_key] = data.value[key];
      keys[new_key] = new_key;
      delete data.value[key];
      delete keys[key];
    }
  }
</script>

<table>
  <th>
    <span>{data.name}</span> <br />
  </th>
  <tbody>
    {#each Object.keys(keys) as key, n}
      {#key key}
        <tr class="table-row">
          <td>
            <input
              bind:value={keys[key]}
              placeholder={key}
              id={`table-key-${n}`}
              on:blur={() => {
                on_key_new_input(key);
              }}
              on:keypress={(e) => {
                if (e.key == "Tab") {
                  on_key_new_input(key);
                }
              }}
            />
          </td>
          <td>
            <input
              bind:value={data.value[key]}
              placeholder={data.value[key].length > 0
                ? data.value[key]
                : "value"}
              id={`table-value-${n}`}
            />
          </td>
          <td>
            <!-- svelte-ignore a11y-missing-attribute -->
            <a
              class="table-trash-icon"
              on:pointerdown={() => {
                delete data.value[key];
                delete keys[key];
                data.value = data.value;
                keys = keys;
              }}><IconTrash /></a
            >
          </td>
        </tr>
      {/key}
    {/each}
    <tr>
      <td
        ><input
          type="text"
          placeholder="key"
          bind:value={key_store}
          on:keypress={(e) => {
            if (e.key == "Enter" || e.key == " ") {
              console.log(data.value);
              console.log(keys);
              key_store = "";
            } else {
              data.value[e.key] = "";
              keys[e.key] = e.key;
              setTimeout(() => {
                document
                  .getElementById(
                    `table-key-${Object.keys(data.value).length - 1}`
                  )
                  ?.focus();
                key_store = "";
              }, 5);
            }
          }}
        /></td
      >
      <td><input type="text" placeholder="value" disabled /></td>
    </tr>
  </tbody>
</table>

<style>
  input {
    color: white;
    background: var(--darkreader-bg--color-surface-100);
    font-size: 110%;
    border-radius: 5px;
    border: 1px solid var(--darkreader-bg--color-surface-400);
    padding: 5px;
  }
  input:focus {
    outline: none;
  }

  .table-trash-icon :global(svg) {
    filter: invert();
    width: 25px;
    height: 25px;
  }
  .table-trash-icon:hover {
    cursor: pointer;
  }
  .table-trash-icon {
    opacity: 0%;
    transition: 0.1s ease;
  }

  .table-row:hover :global(.table-trash-icon) {
    opacity: 100%;
    transition: 0.1s ease;
  }
</style>
