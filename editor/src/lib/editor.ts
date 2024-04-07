import { NodeEditor, type GetSchemes, ClassicPreset } from "rete";
import { AreaPlugin, AreaExtensions } from "rete-area-plugin";
import {
  ConnectionPlugin,
  Presets as ConnectionPresets,
} from "rete-connection-plugin";
import { SveltePlugin, Presets, type SvelteArea2D } from "rete-svelte-plugin";

import { new_node } from "./editor/utils";
import { string_type } from "./editor/basic_types";
import CustomNode from "./editor/CustomNode.svelte";

type Schemes = GetSchemes<
  ClassicPreset.Node,
  ClassicPreset.Connection<ClassicPreset.Node, ClassicPreset.Node>
>;
type AreaExtra = SvelteArea2D<Schemes>;

export async function createEditor(container: HTMLElement) {
  const socket = new ClassicPreset.Socket("socket");

  const editor = new NodeEditor<Schemes>();
  const area = new AreaPlugin<Schemes, AreaExtra>(container);
  const connection = new ConnectionPlugin<Schemes, AreaExtra>();
  const render = new SveltePlugin<Schemes, AreaExtra>();

  AreaExtensions.selectableNodes(area, AreaExtensions.selector(), {
    accumulating: AreaExtensions.accumulateOnCtrl(),
  });

  render.addPreset(
    Presets.classic.setup({
      customize: {
        node() {
          return CustomNode;
        },
      },
    })
  );

  //render.addPreset(Presets.classic.setup());
  connection.addPreset(ConnectionPresets.classic.setup());

  editor.use(area);
  area.use(connection);
  area.use(render);

  AreaExtensions.simpleNodesOrder(area);

  let node_a = string_type("Input", socket);
  let node_b = new_node("Servitor", socket, { a: "string" }, { b: "string" });

  editor.addNode(node_a);
  editor.addNode(node_b);

  setTimeout(() => {
    // wait until nodes rendered because they dont have predefined width and height
    AreaExtensions.zoomAt(area, editor.getNodes());
  }, 10);

  setTimeout(() => {
    const data: any = [];
    const nodes = editor.getNodes();
    const connections = editor.getConnections();

    for (const node of nodes) {
      let connection: any = {};

      for (const conn of connections) {
        if (conn.source === node.id) {
          connection = conn;
          break;
        }
      }

      let node_position = area.nodeViews.get(node.id)?.position;

      data.push({
        id: node.id,
        label: node.label,
        inputs: node.inputs,
        controls: node.controls,
        outputs: node.outputs,
        connection: connection,
        position: [node_position?.x, node_position?.y],
      });
    }

    console.log(data);
  }, 5000);

  return {
    destroy: () => area.destroy(),
  };
}
