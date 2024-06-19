import { new_node } from "./utils";
import { DropDownControl, TableControl } from "$lib/node_graph";
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

// ============ HTTP Request Type ============
export function http_request_basic(socket: ClassicPreset.Socket) {
  let node = new_node("HTTP Request", socket, ["url", "content"], ["output"]);

  let method = new DropDownControl("method", ["get", "post", "put", "delete"]);

  let content_type = new DropDownControl("content type", [
    "application/json",
    "text/plain",
  ]);

  node.addControl("method", method);
  node.addControl("content type", content_type);

  return node;
}

// ============ Table Type ============
export function table_basic(socket: ClassicPreset.Socket) {
  let node = new_node("Table", socket, [], ["output"]);

  node.addControl("data", new TableControl("data", { key: "value" }));

  return node;
}
