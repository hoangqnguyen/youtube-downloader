<script lang="ts">
  import { onMount } from "svelte";
  import { initQueue } from "./lib/stores/queue.svelte";
  import UrlInput from "./lib/components/UrlInput.svelte";
  import QueuePanel from "./lib/components/QueuePanel.svelte";
  import AdvancedPanel from "./lib/components/AdvancedPanel.svelte";
  import FolderPicker from "./lib/components/FolderPicker.svelte";
  import logo from "./assets/logo.png";

  onMount(async () => {
    await initQueue();
  });
</script>

<div class="app">
  <header class="titlebar">
    <div class="logo">
      <img src={logo} alt="Logo" width="24" height="24" />
      <span class="app-name">YT Downloader</span>
    </div>
  </header>

  <main class="content">
    <div class="top-section">
      <UrlInput />
      <FolderPicker />
      <AdvancedPanel />
    </div>

    <div class="divider"></div>

    <QueuePanel />
  </main>
</div>

<style>
  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--bg);
    overflow: hidden;
  }

  .titlebar {
    height: 48px;
    display: flex;
    align-items: center;
    padding: 0 16px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    -webkit-app-region: drag;
    user-select: none;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 10px;
    -webkit-app-region: no-drag;
  }

  .app-name {
    font-size: 14px;
    font-weight: 700;
    color: var(--text);
    letter-spacing: -0.01em;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 16px;
    gap: 12px;
    overflow: hidden;
    min-height: 0;
  }

  .top-section {
    display: flex;
    flex-direction: column;
    gap: 10px;
    flex-shrink: 0;
  }

  .divider {
    height: 1px;
    background: var(--border);
    flex-shrink: 0;
    margin: 0 -4px;
  }
</style>
