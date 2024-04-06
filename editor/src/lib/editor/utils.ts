import { ClassicPreset } from "rete";

export function new_node(
  name: string,
  socket: ClassicPreset.Socket,
  input_structure: any,
  output_structure: any
) {
  const node = new ClassicPreset.Node(name);
  Object.keys(input_structure).forEach((key: string) => {
    node.addInput(key, new ClassicPreset.Input(socket));
  });
  Object.keys(output_structure).forEach((key: string) => {
    node.addOutput(key, new ClassicPreset.Output(socket));
  });

  return node;
}
