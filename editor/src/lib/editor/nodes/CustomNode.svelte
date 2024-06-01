<script lang="ts">
  import type Editor from "$lib/components/editor.svelte";
  import { editor_store } from "$lib/store";
  import type { ClassicScheme, SvelteArea2D } from "rete-svelte-plugin";
  import { Ref } from "rete-svelte-plugin";

  let editor: Editor;
  editor_store.subscribe((value: Editor) => {
    if (value) {
      editor = value;
    }
  });

  type NodeExtraData = { width?: number; height?: number };

  function sortByIndex<K, I extends undefined | { index?: number }>(
    entries: [K, I][]
  ) {
    entries.sort((a, b) => {
      const ai = (a[1] && a[1].index) || 0;
      const bi = (b[1] && b[1].index) || 0;
      return ai - bi;
    });
    return entries as [K, Exclude<I, undefined>][];
  }

  export let data: ClassicScheme["Node"] & NodeExtraData;
  export let emit: (props: SvelteArea2D<ClassicScheme>) => void;

  $: width = Number.isFinite(data.width) ? `${data.width}px` : "";
  $: height = Number.isFinite(data.height) ? `${data.height}px` : "";

  $: inputs = sortByIndex(Object.entries(data.inputs));
  $: controls = sortByIndex(Object.entries(data.controls));
  $: outputs = sortByIndex(Object.entries(data.outputs));
  function any<T>(arg: T): any {
    return arg;
  }
</script>

<div
  class="node {data.selected ? 'selected' : ''}"
  style:width
  style:height
  data-testid="node"
>
  <div class="title" data-testid="title">
    <span>{data.label}</span>

    <span
      on:pointerdown|stopPropagation={() => null}
      style="justify-self: end;"
    >
      <button
        on:click={() => {
          editor.removeNode(data.id);
        }}>X</button
      >
    </span>
  </div>

  <div class="node-body">
    <div class="node-body-inputs">
      <!-- Inputs -->
      {#each inputs as [key, input]}
        <div class="input" data-testid="'input-'+key">
          <Ref
            class="input-socket"
            data-testid="input-socket"
            init={(element) =>
              emit({
                type: "render",
                data: {
                  type: "socket",
                  side: "input",
                  key,
                  nodeId: data.id,
                  element,
                  payload: input.socket,
                },
              })}
            unmount={(ref) => emit({ type: "unmount", data: { element: ref } })}
          />
          {#if !input.control || !input.showControl}
            <div class="input-title" data-testid="input-title">
              {input.label || ""}
            </div>
          {/if}
          {#if input.control && input.showControl}
            <Ref
              class="input-control"
              data-testid="input-control"
              init={(element) =>
                emit({
                  type: "render",
                  data: {
                    type: "control",
                    element,
                    payload: any(input).control,
                  },
                })}
              unmount={(ref) =>
                emit({ type: "unmount", data: { element: ref } })}
            />
          {/if}
        </div>
      {/each}
    </div>

    <!-- Controls -->
    {#each controls as [key, control]}
      <Ref
        class="control"
        data-testid={"control-" + key}
        init={(element) =>
          emit({
            type: "render",
            data: {
              type: "control",
              element,
              payload: control,
            },
          })}
        unmount={(ref) => emit({ type: "unmount", data: { element: ref } })}
      />
    {/each}

    <div class="node-body-outputs">
      <!-- Outputs -->
      {#each outputs as [key, output]}
        <div class="output" data-testid="'output-'+key">
          <div class="output-title" data-testid="output-title">
            {output.label || ""}
          </div>
          <Ref
            class="output-socket"
            data-testid="output-socket"
            init={(element) =>
              emit({
                type: "render",
                data: {
                  type: "socket",
                  side: "output",
                  key,
                  nodeId: data.id,
                  element,
                  payload: output.socket,
                },
              })}
            unmount={(ref) => emit({ type: "unmount", data: { element: ref } })}
          />
        </div>
      {/each}
    </div>
  </div>
</div>

<style lang="scss">
  @use "sass:math";
  @import "../vars.sass";

  .node {
    background: var(--color-surface-200);
    border-radius: 10px;
    cursor: pointer;
    box-sizing: border-box;
    height: auto;
    padding-bottom: 6px;
    position: relative;
    user-select: none;
    transition: 0.2s ease;

    &:hover {
      transition: 0.2s ease;
    }

    /*&.selected {
      border-color: blue; 
    }*/

    .title {
      color: white;
      background: var(--color-surface-300);
      border-top-left-radius: 10px;
      border-top-right-radius: 10px;
      font-family: sans-serif;
      font-size: 18px;
      padding: 8px;
      margin-bottom: 10px;

      display: grid;
      grid-auto-flow: column;
      gap: 10px;
    }

    .title > span > button {
      padding: 5px 7.5px;
      background: none;
      transition: 0.25s ease;
      border: none;
      transform: scaleX(1.1);
    }
    .title > span > button:hover {
      cursor: pointer;
      background: var(--color-surface-400);
      transition: 0.25s ease;
    }

    .output {
      text-align: right;
    }

    .input {
      text-align: left;
    }

    :global(.output-socket) {
      text-align: right;
      margin-right: -20px;
      display: inline-block;
    }

    :global(.input-socket) {
      text-align: left;
      margin-left: -20px;
      display: inline-block;
    }

    :global(.input-socket) > :global(.socket),
    :global(.output-socket) > :global(.socket) {
      background: var(--color-surface-300);
    }

    .input-title,
    .output-title {
      vertical-align: middle;
      color: white;
      display: inline-block;
      font-family: sans-serif;
      font-size: 14px;
      margin: $socket-margin;
      margin-right: -5px;
      line-height: $socket-size;
    }

    .input-title {
      margin-left: -5px;
      margin-right: 0;
    }

    :global(.input-control) {
      z-index: 1;
      width: calc(100% - #{$socket-size + 2 * $socket-margin});
      vertical-align: middle;
      display: inline-block;
    }

    :global(.control) {
      display: block;
      padding: $socket-margin math.div($socket-size, 2) + $socket-margin;
    }
    :global(.control) > :global(input) {
      color: black;
      border-radius: 5px;
    }
  }

  .node-body {
    display: flex;
  }
</style>
