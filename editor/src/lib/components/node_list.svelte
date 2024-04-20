<script lang="ts">
  import type Editor from "$lib/editor";
  import NodeListButton from "./node_list_button.svelte";
  import { editor_store } from "$lib/store";
  import { new_node, type NodeData } from "$lib/editor/utils";
  import {
    string_type,
    number_type,
    math_basic,
  } from "$lib/editor/basic_types";
  import { IconAbc, Icon123, IconCalculator } from "@tabler/icons-svelte";

  let available_nodes = [
    {
      label: "Number",
      node_initializer: number_type,
      icon: Icon123,
    },
    {
      label: "String",
      node_initializer: string_type,
      icon: IconAbc,
    },
    {
      label: "Math",
      node_initializer: math_basic,
      icon: IconCalculator,
    },
  ];
  let nodes: NodeData[] = [];

  let once = false;

  let editor: Editor;
  editor_store.subscribe(async (value) => {
    if (value) {
      editor = value;

      if (!once) {
        await nodes.map(async (node) => {
          let node_to_add = new_node(
            node.label,
            editor.socket,
            node.inputs,
            node.outputs,
            node.id
          );

          await editor.editor.addNode(node_to_add);

          let view = editor.area.nodeViews.get(node_to_add.id);
          await view?.translate(node.position[0] || 0, node.position[1] || 0);
        });

        await nodes.map(async (node) => {
          await editor.editor.addConnection({
            id: node.connection?.id || "",
            source: node.connection?.source || "",
            target: node.connection?.target || "",
            sourceOutput: node.connection?.source_output || "",
            targetInput: node.connection?.target_input || "",
          });
        });

        once = true;
      }
    }
  });
</script>

<div id="node_list_container_parent">
  <span>Nodes</span>
  {#each available_nodes as node}
    <NodeListButton
      onDrop={async (x, y) => {
        let node_to_add = node.node_initializer(editor.socket);
        await editor.editor.addNode(node_to_add);

        editor.area.area.setPointerFrom(
          new MouseEvent("drop", { clientX: x, clientY: y })
        );
        let position = editor.area.area.pointer;

        let view = editor.area.nodeViews.get(node_to_add.id);
        await view?.translate(position.x, position.y);
      }}
    >
      <svelte:component this={node.icon} size={35} />
    </NodeListButton>
  {/each}

  <button
    on:click={() => {
      console.log(editor.toJSON());
    }}>test</button
  >
</div>

<style>
  #node_list_container_parent {
    display: flex;
    flex-direction: column;
    background: var(--color-surface-200);
    height: 100vh;
    width: 75px;
    align-items: center;
  }
</style>