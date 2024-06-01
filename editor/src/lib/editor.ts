import { NodeEditor, type GetSchemes, ClassicPreset } from "rete";
import { AreaPlugin, AreaExtensions } from "rete-area-plugin";
import {
  ConnectionPlugin,
  Presets as ConnectionPresets,
} from "rete-connection-plugin";
import { SveltePlugin, Presets, type SvelteArea2D } from "rete-svelte-plugin";
import CustomNode from "./editor/nodes/CustomNode.svelte";
import { type NodeData, type Connection, type Control } from "./types";
import DropDown from "./editor/nodes/DropDown.svelte";

type Schemes = GetSchemes<
  ClassicPreset.Node,
  ClassicPreset.Connection<ClassicPreset.Node, ClassicPreset.Node>
>;
type AreaExtra = SvelteArea2D<Schemes>;

export class DropDownControl extends ClassicPreset.Control {
  constructor(public name: string, public options: string[], public value = 0) {
    super();
  }
}

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
          control(data) {
            if (data.payload instanceof DropDownControl) {
              return DropDown as any;
            }
            if (data.payload instanceof ClassicPreset.InputControl) {
              return Presets.classic.Control as any;
            }
          },
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
    const data: NodeData[] = [];
    const nodes = this.editor.getNodes();
    const connections = this.editor.getConnections();

    for (const node of nodes) {
      let connection: Connection | undefined;

      for (const conn of connections) {
        if (conn.source === node.id) {
          connection = {
            id: conn.id,
            source: conn.source,
            target: conn.target,
            source_output: conn.sourceOutput,
            target_input: conn.targetInput,
          };
          break;
        }
      }

      let node_position = this.area.nodeViews.get(node.id)?.position;
      let controls: Record<string, Control> = {};
      Object.keys(node.controls).map((key: string) => {
        controls[key] = {
          //@ts-ignore
          name: node.controls[key]?.name,
          //@ts-ignore
          value: node.controls[key]?.value,
        };
      });

      data.push({
        id: node.id,
        label: node.label,
        inputs: Object.values(node.inputs).map((input) => input?.label),
        controls: controls,
        outputs: Object.values(node.outputs).map((output) => output?.label),
        connection: connection,
        position: [node_position?.x, node_position?.y],
      });
    }

    return data;
  }

  removeNode(id: string) {
    let node = this.editor.getNode(id);

    let connections = this.editor.getConnections();
    connections.forEach((conn) => {
      if (conn.source === id || conn.target === id) {
        this.editor.removeConnection(conn.id);
      }
    });

    this.editor.removeNode(id);
  }

  destroy() {
    this.area.destroy();
  }
}
