<script>
  import { IconSetting } from "$lib/icons/icons";

  let graph_list = [
    {
      id: 0,
      content: [],
      generated_javascript: [],
      creation_date: "2024-05-12",
    },
    {
      id: 1,
      content: [],
      generated_javascript: [],
      creation_date: "2024-05-12",
    },
    {
      id: 2,
      content: [],
      generated_javascript: [],
      creation_date: "2024-05-12",
    },
  ];
</script>

<div id="top-bar-container">
  <div id="top-bar">
    <span>Username</span>
    <svelte:component this={IconSetting} />
  </div>
</div>

<div id="graph-list-container">
  <div style="font-size: 50px; align-items: center; justify-items: center;">
    +
  </div>
  {#each graph_list as graph}
    <div class="graph-entry">
      <h1>{graph.id}</h1>
      <span>{graph.creation_date}</span>
    </div>
  {/each}
</div>

<style>
  #top-bar-container {
    background: var(--darkreader-bg--color-surface-200);
    display: flex;
    align-content: center;
    justify-content: flex-end;
    padding: 10px 50px;
  }

  #top-bar-container :global(svg) {
    filter: invert();
    width: 30px;
    height: 30px;
  }

  #top-bar {
    display: flex;
    gap: 20px;
    align-items: center;
  }

  #graph-list-container {
    display: flex;
    flex-wrap: wrap;
    gap: 25px;
    padding: 100px;
    box-sizing: border-box;
  }
  #graph-list-container > div {
    display: grid;
    background: var(--darkreader-bg--color-surface-200);
    transition: 0.25s ease;
    border-radius: 10px;
    padding: 20px;
    width: 150px;
    height: 150px;
  }
  #graph-list-container > div:hover {
    transition: 0.25s ease;
    transform: scale(1.05);
    cursor: pointer;
  }
</style>
