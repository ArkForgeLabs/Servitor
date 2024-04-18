import { new_node } from "./utils";
import { DropDownControl } from "$lib/editor";
import { ClassicPreset } from "rete";

// ============= Primitive Types =============
export function string_type(socket: ClassicPreset.Socket) {
  let node = new_node("String", socket, [], ["output"]);
  node.addControl(
    "value",
    new ClassicPreset.InputControl("text", { initial: "" })
  );

  return node;
}

export function number_type(socket: ClassicPreset.Socket) {
  let node = new_node("Number", socket, [], ["output"]);
  node.addControl(
    "value",
    new ClassicPreset.InputControl("number", { initial: 0 })
  );

  return node;
}

// ============ Math Types =============
export function math_basic(socket: ClassicPreset.Socket) {
  let node = new_node(
    "Math",
    socket,
    ["first number", "second number"],
    ["output"]
  );

  let dropdown_control = new DropDownControl("math", [
    "add",
    "sub",
    "mul",
    "div",
  ]);

  node.addControl("operation", dropdown_control);

  return node;
}
