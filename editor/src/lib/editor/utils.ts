import { ClassicPreset } from "rete";

export type Connection = {
  id: string;
  source: string;
  target: string;
  source_output: string;
  target_input: string;
};

export type NodeData = {
  id: string;
  label: string;
  inputs: (string | undefined)[];
  outputs: (string | undefined)[];
  controls: Map<string, { name: string; value: string | number | boolean }>;
  connection: Connection | undefined;
  position: [number | undefined, number | undefined];
};

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
    node.addInput(key, new ClassicPreset.Input(socket, key));
  });
  output_structure.forEach((key: string) => {
    node.addOutput(key, new ClassicPreset.Output(socket, key));
  });

  return node;
}

export function normalized_to_pixel(
  normalizedValue: number,
  minValue: number,
  maxValue: number
) {
  // Adjust the normalized value to be in the range of 0 to 1
  let adjustedNormalizedValue = (normalizedValue + 1) / 2;

  // Perform the linear transformation
  let number = adjustedNormalizedValue * (maxValue - minValue) + minValue;

  return number;
}
