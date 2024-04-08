import { ClassicPreset } from "rete";

export function new_node(
  name: string,
  socket: ClassicPreset.Socket,
  input_structure: any[],
  output_structure: any[],
  id?: string | undefined
) {
  const node = new ClassicPreset.Node(name);
  if (id) {
    node.id = id;
  }
  input_structure.forEach((key: string) => {
    node.addInput(key, new ClassicPreset.Input(socket));
  });
  output_structure.forEach((key: string) => {
    node.addOutput(key, new ClassicPreset.Output(socket));
  });

  return node;
}
