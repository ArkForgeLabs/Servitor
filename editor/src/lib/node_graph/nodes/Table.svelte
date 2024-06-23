<script lang="ts">
  import { TableControl } from "$lib/node_graph";
  export let data: TableControl = new TableControl("", { key: "value" });
  let keys = {};
  Object.keys(data.value).map((key) => {
    keys[key] = key;
  });

  let key_store = "";
  let value_store = "";

  function on_key_new_input(key: string) {
    let key_not_exist = Object.values(keys).filter(
      (found_key) => found_key == keys[key]
    ).length;

    let new_key = keys[key];
    if (new_key.toString().length > 0 && new_key != key) {
      if (key_not_exist > 1) {
        new_key = `${new_key} (${key_not_exist})`;
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
      <tr>
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
            placeholder={data.value[key].length > 0 ? data.value[key] : "value"}
            id={`table-value-${n}`}
          />
        </td>
        <td>
          <button>-</button>
        </td>
      </tr>
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
      <td
        ><input
          type="text"
          placeholder="value"
          bind:value={value_store}
          on:blur={() => {
            if (value_store.length > 0) {
              data.value[`key${Object.keys(data.value).length}`] = value_store;
              value_store = "";
            }
          }}
        /></td
      >
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

  button {
    background: var(--darkreader-bg--color-surface-400);
    border: none;
    padding: 5px;
    font-size: 110%;
  }
</style>
