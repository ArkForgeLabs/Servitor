<script lang="ts">
  import type Editor from "$lib/editor";
  import NodeListButton from "./node_list_button.svelte";
  import { editor_store } from "$lib/store";

  let editor: Editor;
  editor_store.subscribe((value) => {
    if (value) {
      editor = value;

      editor.area.container.addEventListener("drop", (e) => {
        editor.area.area.setPointerFrom(e);
        let position = editor.area.area.pointer;

        console.log("position", position);
      });
    }
  });
</script>

<div id="node_list_container_parent">
  <NodeListButton
    onDrop={async (x, y) => {
      editor.area.area.setPointerFrom(
        new MouseEvent("drop", { clientX: x, clientY: y })
      );
      let position = editor.area.area.pointer;
      let view = editor.area.nodeViews.get("test");

      await view?.translate(position.x, position.y);
    }}
  >
    <button>Add Node</button>
  </NodeListButton>
  <NodeListButton
    onDrop={async (x, y) => {
      editor.area.area.setPointerFrom(
        new MouseEvent("drop", { clientX: x, clientY: y })
      );
      let position = editor.area.area.pointer;
      let view = editor.area.nodeViews.get("test");

      await view?.translate(position.x, position.y);
    }}
  >
    <button>Add Node</button>
  </NodeListButton>
</div>

<style>
  #node_list_container_parent {
    display: flex;
    flex-direction: column;
    background: var(--color-surface-200);
    height: 100vh;
    width: 150px;
  }
</style>
