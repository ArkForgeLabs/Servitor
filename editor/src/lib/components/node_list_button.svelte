<script lang="ts">
  import { onMount } from "svelte";

  export let left = 0;
  export let top = 0;
  export let onDrop = async (x: number, y: number) => {};

  let moving = false;

  function onMouseDown() {
    moving = true;
  }

  function onMouseMove(e: MouseEvent) {
    if (moving) {
      left += e.movementX;
      top += e.movementY;
    }
  }

  async function onMouseUp(e) {
    moving = false;
    await onDrop(e.clientX, e.clientY);
    left = 0;
    top = 0;
  }

  let element: HTMLElement;

  onMount(() => {
    element.addEventListener("dragover", (e) => e.preventDefault());
    element.addEventListener("drop", async (event) => {
      console.log(event);
    });
  });
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<section
  bind:this={element}
  on:mousedown={onMouseDown}
  on:mouseup={onMouseUp}
  style="left: {left}px; top: {top}px;"
  class="draggable"
>
  <slot />
</section>

<svelte:window on:mousemove={onMouseMove} />

<style>
  .draggable {
    z-index: 2;
    user-select: none;
    cursor: move;
    width: fit-content;
    position: relative;
  }
  .draggable:hover {
    cursor: move;
  }
</style>
