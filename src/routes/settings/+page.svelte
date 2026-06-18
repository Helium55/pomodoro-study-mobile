<script lang="ts">
  import { Download, RotateCcw, Upload, Volume2 } from '@lucide/svelte'
  import { getCopy, LANGUAGE_OPTIONS, normalizeLanguage } from '../../lib/i18n'
  import { ipc } from '../../lib/ipc'
  import { settings } from '../../lib/stores/settings.svelte'

  let soundStatus = $state<'idle' | 'sent' | 'error'>('idle')
  const copy = $derived(getCopy(settings.state.language))

  async function setMin(field: 'workSecs' | 'breakSecs' | 'longBreakSecs', minutes: number) {
    const secs = Math.max(1, Math.min(180, Math.floor(minutes))) * 60
    await settings.update(field, secs)
  }

  async function exportData() {
    const json = await ipc.exportData()
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = `pomodoro-study-${new Date().toISOString().slice(0, 10)}.json`
    link.click()
    URL.revokeObjectURL(url)
  }

  async function importData() {
    const input = document.createElement('input')
    input.type = 'file'
    input.accept = 'application/json'
    input.onchange = async () => {
      const file = input.files?.[0]
      if (!file) return
      const text = await file.text()
      await ipc.importData(text)
      location.reload()
    }
    input.click()
  }

  async function resetData() {
    if (!confirm(copy.settings.resetConfirm)) return
    await ipc.resetData()
    location.reload()
  }

  async function testSound() {
    soundStatus = 'idle'
    try {
      await ipc.notifySound(settings.state.notifySoundFile)
      soundStatus = 'sent'
    } catch {
      soundStatus = 'error'
    }
  }
</script>

<section class="settings-page">
  <header class="page-head">
    <p class="eyebrow">{copy.settings.eyebrow}</p>
    <h1>{copy.settings.title}</h1>
  </header>

  <section class="section">
    <h2>{copy.settings.general}</h2>
    <label>
      {copy.settings.language}
      <select
        value={settings.state.language}
        onchange={(event) =>
          settings.update('language', normalizeLanguage((event.target as HTMLSelectElement).value))}
      >
        {#each LANGUAGE_OPTIONS as option (option.id)}
          <option value={option.id}>{option.label}</option>
        {/each}
      </select>
    </label>
    <p class="github-link">
      <span>{copy.settings.githubLabel}</span>
      <a href="https://github.com/Helium55" target="_blank" rel="noreferrer">github.com/Helium55</a>
    </p>
  </section>

  <section class="section">
    <h2>{copy.settings.timer}</h2>
    <label>
      {copy.settings.workDuration}
      <input
        type="number"
        min="1"
        max="180"
        value={Math.round(settings.state.workSecs / 60)}
        oninput={(event) =>
          setMin('workSecs', Number((event.target as HTMLInputElement).value) || 25)}
      />
    </label>
    <label>
      {copy.settings.shortBreak}
      <input
        type="number"
        min="1"
        max="60"
        value={Math.round(settings.state.breakSecs / 60)}
        oninput={(event) =>
          setMin('breakSecs', Number((event.target as HTMLInputElement).value) || 5)}
      />
    </label>
    <label>
      {copy.settings.longBreak}
      <input
        type="number"
        min="1"
        max="120"
        value={Math.round(settings.state.longBreakSecs / 60)}
        oninput={(event) =>
          setMin('longBreakSecs', Number((event.target as HTMLInputElement).value) || 15)}
      />
    </label>
    <label>
      {copy.settings.longBreakEvery}
      <input
        type="number"
        min="2"
        max="10"
        value={settings.state.longBreakEvery}
        oninput={(event) =>
          settings.update('longBreakEvery', Number((event.target as HTMLInputElement).value) || 4)}
      />
    </label>
    <label class="check">
      <input
        type="checkbox"
        checked={settings.state.autoContinue}
        onchange={(event) =>
          settings.update('autoContinue', (event.target as HTMLInputElement).checked)}
      />
      {copy.settings.autoContinue}
    </label>
  </section>

  <section class="section">
    <h2>{copy.settings.notifications}</h2>
    <label class="check">
      <input
        type="checkbox"
        checked={settings.state.notifySystem}
        onchange={(event) =>
          settings.update('notifySystem', (event.target as HTMLInputElement).checked)}
      />
      {copy.settings.systemToast}
    </label>
    <label class="check">
      <input
        type="checkbox"
        checked={settings.state.notifySound}
        onchange={(event) =>
          settings.update('notifySound', (event.target as HTMLInputElement).checked)}
      />
      {copy.settings.sound}
    </label>
    <label>
      {copy.settings.soundFile}
      <input
        type="text"
        value={settings.state.notifySoundFile}
        oninput={(event) =>
          settings.update('notifySoundFile', (event.target as HTMLInputElement).value)}
      />
    </label>
    <div class="sound-test">
      <button type="button" onclick={testSound}>
        <Volume2 size={17} />
        <span>{copy.settings.testSound}</span>
      </button>
      {#if soundStatus !== 'idle'}
        <p class:error={soundStatus === 'error'} class="sound-status" role="status">
          {soundStatus === 'sent' ? copy.settings.soundSent : copy.settings.soundFailed}
        </p>
      {/if}
    </div>
    <label class="check">
      <input
        type="checkbox"
        checked={settings.state.notifyFullscreen}
        onchange={(event) =>
          settings.update('notifyFullscreen', (event.target as HTMLInputElement).checked)}
      />
      {copy.settings.fullscreenOverlay}
    </label>
    <label class="check">
      <input
        type="checkbox"
        checked={settings.state.notifyTaskbar}
        onchange={(event) =>
          settings.update('notifyTaskbar', (event.target as HTMLInputElement).checked)}
      />
      {copy.settings.taskbarFlash}
    </label>
    <label class="check">
      <input
        type="checkbox"
        checked={settings.state.notifyVibration}
        onchange={(event) =>
          settings.update('notifyVibration', (event.target as HTMLInputElement).checked)}
      />
      {copy.settings.vibration}
    </label>
    <label class="check">
      <input
        type="checkbox"
        checked={settings.state.notifyForeground}
        onchange={(event) =>
          settings.update('notifyForeground', (event.target as HTMLInputElement).checked)}
      />
      {copy.settings.foregroundNotification}
    </label>
    <p class="android-guidance">{copy.settings.androidGuidance}</p>
  </section>

  <section class="section actions">
    <h2>{copy.settings.data}</h2>
    <button type="button" onclick={exportData}>
      <Download size={17} />
      <span>{copy.settings.exportJson}</span>
    </button>
    <button type="button" onclick={importData}>
      <Upload size={17} />
      <span>{copy.settings.importJson}</span>
    </button>
    <button type="button" onclick={resetData}>
      <RotateCcw size={17} />
      <span>{copy.settings.reset}</span>
    </button>
  </section>
</section>

<style>
  .page-head {
    border-bottom: 1px solid var(--color-border);
    padding: 22px clamp(18px, 4vw, 34px);
  }

  .eyebrow {
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 3px;
    margin: 0;
  }

  h1,
  h2 {
    margin: 0;
    font-family: var(--font-display);
  }

  h1 {
    font-size: clamp(30px, 5vw, 54px);
    margin-top: 7px;
  }

  h2 {
    color: var(--color-accent);
    font-size: 17px;
  }

  .section {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 14px;
    border-bottom: 1px solid var(--color-border);
    padding: 20px;
  }

  .section h2 {
    grid-column: 1 / -1;
  }

  label {
    display: grid;
    gap: 7px;
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 11px;
    letter-spacing: 2px;
    text-transform: uppercase;
  }

  .check {
    align-items: center;
    display: flex;
    gap: 10px;
  }

  input,
  select {
    min-width: 0;
    border: var(--border-width) solid var(--color-border);
    background: var(--color-bg-elevated);
    color: var(--color-fg);
    padding: 10px 12px;
  }

  input[type='checkbox'] {
    accent-color: var(--color-accent);
    width: 18px;
    height: 18px;
  }

  button {
    min-height: 44px;
    border: var(--border-width) solid var(--color-fg);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 900;
    letter-spacing: 2px;
  }

  button:hover {
    background: var(--color-accent);
    border-color: var(--color-accent);
    color: var(--color-bg);
  }

  .sound-test {
    display: grid;
    gap: 8px;
  }

  .sound-status {
    margin: 0;
    color: var(--color-accent);
    font-family: var(--font-mono);
    font-size: 11px;
    letter-spacing: 1px;
  }

  .sound-status.error {
    color: var(--color-fg);
  }

  .github-link {
    align-self: end;
    margin: 0;
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 1px;
  }

  .github-link a {
    color: var(--color-fg-muted);
    text-decoration: underline;
    text-underline-offset: 3px;
  }

  .github-link a:hover {
    color: var(--color-accent);
  }

  .android-guidance {
    grid-column: 1 / -1;
    margin: 0;
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 11px;
    letter-spacing: 1px;
  }

  @media (width <= 760px) {
    .page-head {
      padding: 18px 16px;
    }

    .section {
      grid-template-columns: 1fr;
      padding: 16px;
    }
  }
</style>
