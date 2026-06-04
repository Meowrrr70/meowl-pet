<script>
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import meowlImg from "./assets/meowl.png";

  let wordCount = 0;
  let wiggle = false;

  function onWord(count) {
    wordCount = count;
    wiggle = true;
    setTimeout(() => (wiggle = false), 400);
  }

  onMount(async () => {
    wordCount = await invoke("get_word_count");
    await listen("word-count", (e) => onWord(e.payload));
  });

  function close() {
    getCurrentWindow().close();
  }
</script>

<main>
  <button class="close" on:click={close}>×</button>
  <img class:wiggle src={meowlImg} alt="Meowl" draggable="false" />
  <p>{wordCount} words</p>
</main>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    background: transparent;
    overflow: hidden;
  }

  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 8px 8px 12px;
    cursor: default;
    user-select: none;
    position: relative;
  }

  img {
    width: 160px;
    height: auto;
    pointer-events: none;
  }

  p {
    color: white;
    font-family: sans-serif;
    font-size: 16px;
    font-weight: bold;
    text-shadow: 0 1px 3px black, 0 0 6px black;
    margin-top: 6px;
  }

  .close {
    position: absolute;
    top: 4px;
    right: 4px;
    width: 22px;
    height: 22px;
    border: none;
    border-radius: 50%;
    background: rgba(0, 0, 0, 0.5);
    color: white;
    font-size: 14px;
    line-height: 1;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.15s;
    z-index: 10;
  }

  main:hover .close {
    opacity: 1;
  }

  @keyframes wiggle {
    0%,
    100% {
      transform: rotate(0deg);
    }
    25% {
      transform: rotate(-6deg);
    }
    75% {
      transform: rotate(6deg);
    }
  }

  .wiggle {
    animation: wiggle 0.4s ease-in-out;
  }
</style>
