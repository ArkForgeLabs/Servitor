/*
 Generated by typeshare 1.9.2
*/

export type OptionalString = string | undefined;

export type OptionalNumber = number | undefined;

export interface Connection {
	id: string;
	source: string;
	target: string;
	source_output: string;
	target_input: string;
}

export interface Control {
	name: string;
	value: string;
}

export interface NodeData {
	id: string;
	label: string;
	inputs: OptionalString[];
	outputs: OptionalString[];
	controls: Record<string, Control>;
	position: [OptionalNumber, OptionalNumber];
	connection?: Connection;
}
