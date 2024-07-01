<script lang="ts">
  import { IconRightArrow } from "$lib/icons/icons";

  let email = "";
  let username = "";
  let password = "";

  let error_message = "";

  let is_signup = false;
  let is_error = false;

  const validateEmail = (email: string) => {
    return String(email)
      .toLowerCase()
      .match(
        /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|.(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
      );
  };

  const validatePassword = (password: string) => {
    return /^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/.test(
      password
    );
  };

  function validate_input() {
    if (
      (!is_signup && (!email || !password)) ||
      (is_signup && (!email || !password || !username))
    ) {
      error_message = "Please fill all the fields";
      is_error = true;
      return false;
    }

    if (!validateEmail(email)) {
      error_message = "Invalid E-Mail";
      is_error = true;
      return false;
    }
    if (!validatePassword(password)) {
      error_message =
        "Invalid password, make sure it has upper and lower case letters, a number and a special character!";
      is_error = true;
      return false;
    }

    return true;
  }

  function submit() {
    if (validate_input()) {
      if (is_signup) {
        fetch("/apiv1_auth/create_account", {
          method: "POST",
          headers: [["Content-Type", "application/json"]],
          body: JSON.stringify({
            email,
            username,
            password,
          }),
        })
          .then((res) => {
            if (res.status == 200) {
              window.open("/dashboard", "_self");
            } else {
              error_message =
                "Couldn't sign up at this moment, please try again later!";
              is_error = true;
            }
          })
          .catch(() => {
            error_message =
              "Couldn't sign up at this moment, please try again later!";
            is_error = true;
          });
      } else {
        fetch(
          `/apiv1_auth/login?email=${encodeURIComponent(email)}&password=${encodeURIComponent(password)}`
        )
          .then((res) => {
            if (res.status == 200) {
              window.open("/dashboard", "_self");
            } else {
              error_message = "invalid email or password";
              is_error = true;
            }
          })
          .catch(() => {
            error_message = "invalid email or password";
            is_error = true;
          });
      }
    }
  }
</script>

<section aria-label="landing">
  <div id="landing_content_container">
    <img src="/owl.svg" alt="logo" />

    <div id="content-container">
      <div id="content">
        <span id="content-heading"
          >{is_signup ? "Get Started!" : "Welcome back!"}</span
        >
        <br />
        <div id="input">
          <label for="email">Email</label>
          <input
            id="email"
            type="text"
            bind:value={email}
            on:keypress={(e) => {
              if (e.key === "Enter") {
                submit();
              }
            }}
          />
          {#if is_signup}
            <div><label for="username">Username</label></div>
            <input
              id="username"
              type="username"
              bind:value={username}
              on:keypress={(e) => {
                if (e.key === "Enter") {
                  submit();
                }
              }}
            />
          {/if}
          <div><label for="password">Password</label></div>
          <input
            id="password"
            type="password"
            bind:value={password}
            on:keypress={(e) => {
              if (e.key === "Enter") {
                submit();
              }
            }}
          />
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-missing-attribute -->
          <a
            id="submit_button"
            style="text-decoration: none;"
            on:click={submit}
          >
            {is_signup ? "Sign Up" : "Log In"}
            <svelte:component this={IconRightArrow} />
          </a>

          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-missing-attribute -->
          <a style="font-size: medium;">
            {is_signup
              ? "By signing up, you accept our license"
              : "Forgot password?"}
          </a>

          {#if is_error}
            <span style="color: #ff2d2d;">{error_message}</span>
          {/if}

          <br />

          <div id="signup">
            <hr />
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-missing-attribute -->
            <a
              on:click={() => {
                is_signup = is_signup ? false : true;
              }}>{is_signup ? "Log In" : "Sign Up"}</a
            >
            <hr />
          </div>
        </div>
      </div>
      <div />
    </div>
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
    background: var(--darkreader-bg--color-surface-200);
    margin: auto auto;
    width: 80%;
  }

  #side {
    height: 100vh;
    width: 50vw;
    object-fit: cover;
  }

  #content-heading {
    margin: 0;
    font-size: 75px;
    font-family: serif;
    font-style: italic;
    font-weight: bolder;
    line-height: normal;
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
    background: var(--darkreader-bg--color-surface-300);
    border-radius: 5px;
    padding: 5px;
  }

  #input {
    display: grid;
    transition: 0.25s ease;
    width: 300px;
    gap: 10px;
    font-size: larger;
  }
  #input:hover {
    transition: 0.25s ease;
  }

  #input :global(a) {
    text-decoration: underline;
    color: white;
    width: fit-content;
  }
  #input :global(a):hover {
    cursor: pointer;
  }

  #submit_button :global(svg) {
    height: 25px;
    width: 25px;
    transition: 0.25s ease;
    filter: invert();
    font-size: larger;
  }

  #submit_button {
    display: grid;
    grid-auto-flow: column;
    justify-items: right;
    padding: 5px;
    border-radius: 5px;
    background: var(--darkreader-bg--color-surface-300);
    color: white;
    font-weight: bold;
    filter: invert();
    width: 100%;
  }

  #submit_button:hover :global(svg) {
    cursor: pointer;
    transform: translateX(5px);
    transition: 0.25s ease;
  }

  #signup {
    display: flex;
    gap: 10px;
    font-size: large;
  }

  #signup > hr {
    flex-grow: 1;
    height: 0;
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

    #content {
      width: 300px;
      margin: auto auto;
    }

    #content-heading {
      font-size: 47.5px;
    }

    #content-container {
      background: var(--darkreader-bg--color-surface-200);
      width: 100vw;
      padding-top: 50px;
    }

    #landing_content_container {
      width: 100vw;
      padding: 0;
      background: none;
      z-index: 1;
    }

    #landing_content_container > img {
      margin-top: 10px;
      margin-left: 10px;
      filter: invert();
    }
  }
</style>
