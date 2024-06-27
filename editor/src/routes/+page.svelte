<script lang="ts">
  let email = "";

  function submit() {}
</script>

<section aria-label="landing">
  <div id="landing_content_container">
    <img src="/owl.svg" alt="logo" />

    <div id="content">
      <span id="content-heading">Welcome back!</span>
      <span>Want to see what we're up to?</span>
      <div id="input">
        <input
          id="email"
          type="text"
          placeholder="email"
          bind:value={email}
          on:keypress={(e) => {
            if (e.key === "Enter") {
              submit();
            }
          }}
        />
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
        <img src="/icons/arrow-right.svg" alt="arrow-right" on:click={submit} />
      </div>
    </div>
    <div></div>
  </div>
  <img id="side" src="/images/background.webp" alt="side" />
</section>

<style>
  section {
    width: 100vw;
    height: 100vh;
    display: grid;
    grid-auto-flow: column;
    background: var(--darkreader-bg--color-surface-200);
  }

  #landing_content_container {
    width: 50vw;
    padding: 10px;
    display: grid;
    box-shadow: inset;
  }

  #landing_content_container > img {
    width: 50px;
    height: 50px;
  }

  #content {
    align-self: center;
    justify-self: center;
    display: flex;
    flex-direction: column;
    font-family: sans-serif;
  }

  #side {
    height: 100vh;
    width: 50vw;
    object-fit: cover;
  }

  #content-heading {
    margin: 0;
    font-size: 75px;
    height: 50px;
    font-family: serif;
    font-style: italic;
    font-weight: bolder;
  }

  span {
    font-size: large;
    margin-bottom: 10px;
  }

  input {
    border: none;
    outline: none;
    width: 100%;
    font-size: large;
  }

  #input {
    display: flex;
    box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.5);
    transition: 0.25s ease;
    width: 250px;
    background: white;
  }
  #input:hover {
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.5);
    transition: 0.25s ease;
  }

  #input > img {
    height: 25px;
  }

  #input > img:hover {
    cursor: pointer;
    transition: 0.25s ease;
  }

  @media only screen and (max-width: 768px) {
    #side {
      position: fixed;
      left: 0;
      top: 0;
      width: 100vw;
      height: 50vh;
      object-position: right;
    }

    #landing_content_container {
      color: black;
      width: 100vw;
      background: none;
      z-index: 1;
    }
  }
</style>
