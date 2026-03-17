<script lang="ts">
  import {
    getAudioOnly,
    getResolution,
    getTranscript,
    getMaxConcurrent,
    getAutoRetry,
    getAutoRetryMaxAttempts,
    getAutoRetryDelaySec,
    getCookieBrowser,
    setAudioOnly,
    setResolution,
    setTranscript,
    setMaxConcurrent,
    setAutoRetry,
    setAutoRetryMaxAttempts,
    setAutoRetryDelaySec,
    setCookieBrowser,
  } from "../stores/settings.svelte";

  let audioOnly = $derived(getAudioOnly());
  let resolution = $derived(getResolution());
  let transcriptMode = $derived(getTranscript());
  let maxConcurrent = $derived(getMaxConcurrent());
  let autoRetry = $derived(getAutoRetry());
  let autoRetryMaxAttempts = $derived(getAutoRetryMaxAttempts());
  let autoRetryDelaySec = $derived(getAutoRetryDelaySec());
  let cookieBrowser = $derived(getCookieBrowser());
  import type { Resolution, TranscriptMode, CookieBrowser } from "../types";

  const cookieBrowsers: { value: CookieBrowser; label: string }[] = [
    { value: "none", label: "None" },
    { value: "firefox", label: "Firefox" },
  ];

  const transcriptModes: { value: TranscriptMode; label: string }[] = [
    { value: "none", label: "None" },
    { value: "include", label: "Include" },
    { value: "only", label: "Only" },
  ];

  const resolutions: { value: Resolution; label: string }[] = [
    { value: "best", label: "Best" },
    { value: "1080", label: "1080p" },
    { value: "720", label: "720p" },
    { value: "480", label: "480p" },
    { value: "360", label: "360p" },
  ];

</script>

<details class="advanced-panel">
  <summary class="summary">
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="3"/>
      <path d="M19.07 4.93a10 10 0 0 1 0 14.14M4.93 4.93a10 10 0 0 0 0 14.14"/>
    </svg>
    Advanced options
    <svg class="chevron" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
      <polyline points="6 9 12 15 18 9"/>
    </svg>
  </summary>

  <div class="content">
    <!-- Row 1: Audio only toggle + Resolution -->
    <div class="inline-row">
      <label class="mini-toggle" for="audio-only">
        <div class="toggle" class:on={audioOnly}>
          <input
            id="audio-only"
            type="checkbox"
            checked={audioOnly}
            onchange={(e) => setAudioOnly((e.target as HTMLInputElement).checked)}
          />
          <div class="track"><div class="thumb"></div></div>
        </div>
        <span class="mini-toggle-text">Audio only</span>
      </label>

      <div class="field flex-1" class:disabled={audioOnly}>
        <span class="field-label">Resolution</span>
        <div class="radio-group">
          {#each resolutions as r}
            <label class="radio-option" class:selected={resolution === r.value}>
              <input
                type="radio"
                name="resolution"
                value={r.value}
                checked={resolution === r.value}
                onchange={() => setResolution(r.value)}
                disabled={audioOnly}
              />
              <span class="radio-label">{r.label}</span>
            </label>
          {/each}
        </div>
      </div>
    </div>

    <!-- Row 2: Transcript + Browser cookies -->
    <div class="inline-row">
      <div class="field flex-1">
        <span class="field-label">Transcript</span>
        <div class="radio-group">
          {#each transcriptModes as t}
            <label class="radio-option" class:selected={transcriptMode === t.value}>
              <input
                type="radio"
                name="transcript"
                value={t.value}
                checked={transcriptMode === t.value}
                onchange={() => setTranscript(t.value)}
              />
              <span class="radio-label">{t.label}</span>
            </label>
          {/each}
        </div>
      </div>

      <div class="field flex-1">
        <span class="field-label">Browser cookies</span>
        <div class="radio-group">
          {#each cookieBrowsers as b}
            <label class="radio-option" class:selected={cookieBrowser === b.value}>
              <input
                type="radio"
                name="cookie-browser"
                value={b.value}
                checked={cookieBrowser === b.value}
                onchange={() => setCookieBrowser(b.value)}
              />
              <span class="radio-label">{b.label}</span>
            </label>
          {/each}
        </div>
      </div>
    </div>

    <!-- Row 3: Parallel downloads slider -->
    <div class="field">
      <div class="slider-row">
        <span class="field-label">Parallel downloads</span>
        <input
          type="range"
          min="1"
          max="5"
          value={maxConcurrent}
          onchange={(e) => setMaxConcurrent(Number((e.target as HTMLInputElement).value))}
        />
        <span class="slider-val">{maxConcurrent}</span>
      </div>
    </div>

    <div class="divider"></div>

    <!-- Row 4: Auto-retry toggle + inline sliders -->
    <div class="inline-row">
      <label class="mini-toggle" for="auto-retry">
        <div class="toggle" class:on={autoRetry}>
          <input
            id="auto-retry"
            type="checkbox"
            checked={autoRetry}
            onchange={(e) => setAutoRetry((e.target as HTMLInputElement).checked)}
          />
          <div class="track"><div class="thumb"></div></div>
        </div>
        <span class="mini-toggle-text">Auto-retry</span>
      </label>

      {#if autoRetry}
        <div class="field flex-1">
          <div class="slider-row">
            <span class="field-label">Retries</span>
            <input
              type="range"
              min="1"
              max="10"
              value={autoRetryMaxAttempts}
              onchange={(e) => setAutoRetryMaxAttempts(Number((e.target as HTMLInputElement).value))}
            />
            <span class="slider-val">{autoRetryMaxAttempts}</span>
          </div>
        </div>

        <div class="field flex-1">
          <div class="slider-row">
            <span class="field-label">Delay</span>
            <input
              type="range"
              min="5"
              max="60"
              step="5"
              value={autoRetryDelaySec}
              onchange={(e) => setAutoRetryDelaySec(Number((e.target as HTMLInputElement).value))}
            />
            <span class="slider-val">{autoRetryDelaySec}s</span>
          </div>
        </div>
      {/if}
    </div>
  </div>
</details>

<style>
  .advanced-panel {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }

  .summary {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    cursor: pointer;
    user-select: none;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-dim);
    letter-spacing: 0.03em;
    list-style: none;
    transition: color 0.15s;
  }

  .summary::-webkit-details-marker {
    display: none;
  }

  .summary:hover {
    color: var(--text);
  }

  .chevron {
    margin-left: auto;
    transition: transform 0.2s ease;
  }

  details[open] .chevron {
    transform: rotate(180deg);
  }

  .content {
    padding: 2px 12px 10px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    border-top: 1px solid var(--border);
  }

  .inline-row {
    display: flex;
    align-items: flex-end;
    gap: 12px;
  }

  .flex-1 {
    flex: 1;
    min-width: 0;
  }

  .disabled {
    opacity: 0.35;
    pointer-events: none;
  }

  .divider {
    height: 1px;
    background: var(--border);
    margin: 0;
  }

  .mini-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    flex-shrink: 0;
    padding-bottom: 1px;
  }

  .mini-toggle-text {
    font-size: 11px;
    font-weight: 500;
    color: var(--text-dim);
    white-space: nowrap;
  }

  .toggle input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .track {
    width: 32px;
    height: 18px;
    background: var(--border);
    border-radius: 999px;
    position: relative;
    transition: background 0.2s;
    cursor: pointer;
    flex-shrink: 0;
  }

  .toggle.on .track {
    background: var(--orange);
  }

  .thumb {
    width: 14px;
    height: 14px;
    background: #fff;
    border-radius: 50%;
    position: absolute;
    top: 2px;
    left: 2px;
    transition: transform 0.2s;
    box-shadow: 0 1px 2px rgba(0,0,0,0.3);
  }

  .toggle.on .thumb {
    transform: translateX(14px);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .field-label {
    font-size: 11px;
    font-weight: 500;
    color: var(--text-dim);
    white-space: nowrap;
  }

  .radio-group {
    display: flex;
    gap: 0;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .radio-option {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    position: relative;
  }

  .radio-option + .radio-option {
    border-left: 1px solid var(--border);
  }

  .radio-option input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .radio-label {
    display: block;
    width: 100%;
    text-align: center;
    padding: 4px 3px;
    font-size: 11px;
    font-weight: 500;
    color: var(--text-dim);
    transition: all 0.15s;
  }

  .radio-option.selected .radio-label {
    background: var(--orange);
    color: #fff;
    font-weight: 600;
  }

  .radio-option:not(.selected):hover .radio-label {
    background: var(--bg-hover);
    color: var(--text);
  }

  .slider-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  input[type="range"] {
    flex: 1;
    height: 3px;
    appearance: none;
    background: var(--border);
    border-radius: 999px;
    outline: none;
    border: none;
    cursor: pointer;
  }

  input[type="range"]::-webkit-slider-thumb {
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--orange);
    cursor: pointer;
    box-shadow: 0 1px 2px rgba(0,0,0,0.3);
  }

  .slider-val {
    min-width: 24px;
    text-align: center;
    font-size: 11px;
    font-weight: 600;
    color: var(--orange);
    font-family: var(--font-mono);
  }
</style>
