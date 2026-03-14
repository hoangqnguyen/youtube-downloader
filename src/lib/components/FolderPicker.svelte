<script lang="ts">
  import { getOutputDir, setOutputDir } from "../stores/settings.svelte";

  let outputDir = $derived(getOutputDir());
  import { pickFolder, openFolder } from "../tauri";

  async function browse() {
    const dir = await pickFolder();
    if (dir) setOutputDir(dir);
  }

  function displayPath(p: string) {
    if (!p) return "Loading...";
    // Show last 2 segments for brevity
    const parts = p.replace(/\\/g, "/").split("/").filter(Boolean);
    if (parts.length <= 2) return p;
    return "…/" + parts.slice(-2).join("/");
  }
</script>

<div class="folder-picker">
  <span class="label">Save to</span>
  <div class="path-row">
    <div class="path" title={outputDir}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" opacity="0.5">
        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
      </svg>
      <span class="truncate">{displayPath(outputDir)}</span>
    </div>
    <div class="path-actions">
      <button class="btn-ghost icon-btn" onclick={browse} title="Change folder">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
        </svg>
      </button>
      <button
        class="btn-ghost icon-btn"
        onclick={() => openFolder(outputDir)}
        title="Open folder"
        disabled={!outputDir}
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
          <polyline points="15 3 21 3 21 9"/>
          <line x1="10" y1="14" x2="21" y2="3"/>
        </svg>
      </button>
    </div>
  </div>
</div>

<style>
  .folder-picker {
    display: flex;
    align-items: center;
    gap: 10px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 8px 12px;
  }

  .label {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .path-row {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .path {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-dim);
    font-family: var(--font-mono);
  }

  .path-actions {
    display: flex;
    gap: 2px;
    flex-shrink: 0;
  }

  .icon-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border-radius: var(--radius-sm);
  }

  .icon-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }
</style>
