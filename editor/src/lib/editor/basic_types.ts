import { new_node } from "./utils";
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
function math_node(name: string, socket: ClassicPreset.Socket) {
  let node = new_node(
    name,
    socket,
    ["first number", "second number"],
    ["output"]
  );

  return node;
}

export function add_type(socket: ClassicPreset.Socket) {
  return math_node("Add", socket);
}
export function subtract_type(socket: ClassicPreset.Socket) {
  return math_node("Subtract", socket);
}
export function multiply_type(socket: ClassicPreset.Socket) {
  return math_node("Multiply", socket);
}
export function divide_type(socket: ClassicPreset.Socket) {
  return math_node("Divide", socket);
}
