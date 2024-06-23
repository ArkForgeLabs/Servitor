<script lang="ts">
  import type Editor from "$lib/node_graph";
  import NodeListButton from "./node_list_button.svelte";
  import { editor_store } from "$lib/store";
  import { new_node } from "$lib/node_graph/utils";
  import { type NodeData } from "$lib/types";
  import {
    string_type,
    number_type,
    math_basic,
    http_request_basic,
    table_basic,
  } from "$lib/node_graph/basic_types";

  import {
    Icon123,
    IconAbc,
    IconCalculator,
    IconWeb,
    IconTable,
  } from "$lib/icons/icons";

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
    {
      label: "HTTP Request",
      node_initializer: http_request_basic,
      icon: IconWeb,
    },
    {
      label: "Table",
      node_initializer: table_basic,
      icon: IconTable,
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
      <svelte:component this={node.icon} />
    </NodeListButton>
  {/each}
</div>

<style>
  #node_list_container_parent {
    display: flex;
    flex-direction: column;
    background: var(--darkreader-bg--color-surface-200);
    height: 100vh;
    width: 75px;
    align-items: center;
    padding-top: 25px;
  }

  #node_list_container_parent :global(svg) {
    filter: invert();
  }
</style>
