import { NodeData, type Control } from "../editor/src/lib/types.ts";

//type NodeInformation = NodeData & { children: NodeInformation[] };
type NodeWithChildren = NodeData & { children: NodeWithChildren[] };
type NeoNode = NodeData & { depth: number };

function formatControls(
  // deno-lint-ignore no-explicit-any
  controls: Record<string, any>
): Record<string, Control> {
  const formattedControls: Record<string, Control> = {};
  for (const key in controls) {
    formattedControls[key] = {
      name: key,
      value: controls[key].value as string,
    };
  }
  return formattedControls;
}

// deno-lint-ignore no-explicit-any
function processNodes(nodes: any): [NeoNode[], NodeWithChildren[]] {
  const neoNodes: NeoNode[] = [];
  const nodesWithChildren: NodeWithChildren[] = [];

  // deno-lint-ignore no-explicit-any
  nodes.forEach((node: any) => {
    const formattedControls = formatControls(node.controls);
    const baseNode = {
      id: node.id,
      label: node.label,
      inputs: node.inputs,
      outputs: node.outputs,
      controls: formattedControls,
      position: [node.position[0], node.position[1]],
      connection: node.connection,
    };

    neoNodes.push({
      ...baseNode,
      position: [baseNode.position[0], baseNode.position[1]],
      depth: 0,
    });
    nodesWithChildren.push({
      ...baseNode,
      position: [baseNode.position[0], baseNode.position[1]],
      children: [],
    });
  });

  return [neoNodes, nodesWithChildren];
}

function buildTree(nodes: NodeWithChildren[]): NodeWithChildren {
  const tree = nodes[0];

  function walkTree(
    foundNode: NodeWithChildren,
    node: NodeWithChildren,
    parentID: string
  ): void {
    if (foundNode.id === parentID) {
      foundNode.children.push(node);
    } else {
      for (const child of foundNode.children) {
        walkTree(child, node, parentID);
      }
    }
  }

  for (const node of nodes) {
    if (!node.connection) continue;

    const parentID = node.connection!.target;

    walkTree(tree, node, parentID);
  }

  return tree;
}

function generateJavaScriptCode(node: NodeWithChildren): string {
  let output = "}\nmain();";

  function walkTreeAndGenerateCode(node: NodeWithChildren) {
    const childrenIDs = node.children.map((childNode) => childNode.id);

    switch (node.label) {
      case "Number":
        output =
          ` let var${node.id} = ${node.controls.value.value};\n` + output;
        break;
      case "Math": {
        const operations = ["+", "-", "*", "/"];
        const operationIndex = node.controls.operation
          .value as unknown as number;
        const operation = operations[operationIndex] || "+";
        output =
          ` let var${node.id} = var${childrenIDs[0]} ${operation} var${childrenIDs[1]};\n` +
          output;
        break;
      }
    }

    node.children.forEach((childNode) => {
      walkTreeAndGenerateCode(childNode);
    });
  }

  walkTreeAndGenerateCode(node);

  return `function main() {\n` + output;
}

export function verify_json(input: Object, associated_keys: string[]): boolean {
  let verified = true;
  Object.keys(input).forEach((key) => {
    if (!associated_keys.includes(key)) {
      verified = false;
    }
  });

  return verified;
}
