import { new_node } from "./utils";
import { ClassicPreset } from "rete";

export function string_type(name: string, socket: ClassicPreset.Socket) {
  let node = new_node(name, socket, {}, { output: "string" });
  node.addControl(
    "value",
    new ClassicPreset.InputControl("text", { initial: "" })
  );

  return node;
}
