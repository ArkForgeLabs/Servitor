import { NodeEditor, type GetSchemes, ClassicPreset } from "rete";
import { AreaPlugin, AreaExtensions } from "rete-area-plugin";
import {
  ConnectionPlugin,
  Presets as ConnectionPresets,
} from "rete-connection-plugin";
import { SveltePlugin, Presets, type SvelteArea2D } from "rete-svelte-plugin";
import CustomNode from "./editor/nodes/CustomNode.svelte";
import { new_node, type NodeData, type Connection } from "./editor/utils";
import DropDown from "./editor/nodes/DropDown.svelte";

type Schemes = GetSchemes<
  ClassicPreset.Node,
  ClassicPreset.Connection<ClassicPreset.Node, ClassicPreset.Node>
>;
type AreaExtra = SvelteArea2D<Schemes>;

export class DropDownControl extends ClassicPreset.Control {
  constructor(public name: string, public options: string[]) {
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

    /*let node = new_node(
      "test",
      this.socket,
      ["first"],
      ["output", "second"],
      "test"
    );

    let dropdown_control = new DropDownControl("dropdown", [
      "Option 1",
      "Option 2",
      "Option 3",
    ]);
    node.addControl("dropdown", dropdown_control);

    this.editor.addNode(node); */

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

      data.push({
        id: node.id,
        label: node.label,
        inputs: JSON.parse(JSON.stringify(node.inputs)),
        controls: JSON.parse(JSON.stringify(node.controls)),
        outputs: JSON.parse(JSON.stringify(node.outputs)),
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

/*let node = new_node("test", socket, [], ["output"]);
  editor.addNode(node);

  area.container.addEventListener("click", async (e) => {
    console.log(e);

    area.area.setPointerFrom(e);

    let position = area.area.pointer;
    let view = area.nodeViews.get(node.id);

    await view?.translate(position.x, position.y);
  }); */
