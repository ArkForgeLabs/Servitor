import { NodeEditor, type GetSchemes, ClassicPreset } from "rete";
import { AreaPlugin, AreaExtensions } from "rete-area-plugin";
import {
  ConnectionPlugin,
  Presets as ConnectionPresets,
} from "rete-connection-plugin";
import { SveltePlugin, Presets, type SvelteArea2D } from "rete-svelte-plugin";
import CustomNode from "./editor/CustomNode.svelte";

type Schemes = GetSchemes<
  ClassicPreset.Node,
  ClassicPreset.Connection<ClassicPreset.Node, ClassicPreset.Node>
>;
type AreaExtra = SvelteArea2D<Schemes>;

export default class Editor {
  socket = new ClassicPreset.Socket("socket");
  editor = new NodeEditor<Schemes>();
  connection = new ConnectionPlugin<Schemes, AreaExtra>();
  render = new SveltePlugin<Schemes, AreaExtra>();
  area: AreaPlugin<Schemes, AreaExtra>;

  constructor(container: HTMLElement) {
    this.area = new AreaPlugin<Schemes, AreaExtra>(container);

    AreaExtensions.selectableNodes(this.area, AreaExtensions.selector(), {
      accumulating: AreaExtensions.accumulateOnCtrl(),
    });

    this.render.addPreset(
      Presets.classic.setup({
        customize: {
          node() {
            return CustomNode;
          },
        },
      })
    );

    //render.addPreset(Presets.classic.setup());
    this.connection.addPreset(ConnectionPresets.classic.setup());

    this.editor.use(this.area);
    this.area.use(this.connection);
    this.area.use(this.render);

    AreaExtensions.simpleNodesOrder(this.area);

    setTimeout(() => {
      // wait until nodes rendered because they dont have predefined width and height
      AreaExtensions.zoomAt(this.area, this.editor.getNodes());
    }, 10);
  }

  toJSON() {
    const data: any = [];
    const nodes = this.editor.getNodes();
    const connections = this.editor.getConnections();

    for (const node of nodes) {
      let connection: any = {};

      for (const conn of connections) {
        if (conn.source === node.id) {
          connection = conn;
          break;
        }
      }

      let node_position = this.area.nodeViews.get(node.id)?.position;

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

    return data;
  }

  destroy() {
    this.area.destroy();
  }
}
