<script lang="ts">
  import { onMount } from "svelte";
  import { initQueue } from "./lib/stores/queue.svelte";
  import UrlInput from "./lib/components/UrlInput.svelte";
  import QueuePanel from "./lib/components/QueuePanel.svelte";
  import AdvancedPanel from "./lib/components/AdvancedPanel.svelte";
  import FolderPicker from "./lib/components/FolderPicker.svelte";

  onMount(async () => {
    await initQueue();
  });
</script>

<div class="app">
  <header class="titlebar">
    <div class="logo">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="var(--orange)">
        <path d="M19.59 6.69a4.83 4.83 0 0 1-3.77-4.25V2h-3.45v13.67a2.89 2.89 0 0 1-2.88 2.5 2.89 2.89 0 0 1-2.89-2.89 2.89 2.89 0 0 1 2.89-2.89c.28 0 .54.04.79.1V9.01a6.33 6.33 0 0 0-.79-.05 6.34 6.34 0 0 0-6.34 6.34 6.34 6.34 0 0 0 6.34 6.34 6.34 6.34 0 0 0 6.33-6.34V8.69a8.17 8.17 0 0 0 4.77 1.52V6.76a4.85 4.85 0 0 1-1-.07z"/>
      </svg>
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
