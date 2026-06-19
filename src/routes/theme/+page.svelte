<script lang="ts">
  import { getCopy } from '../../lib/i18n'
  import { settings } from '../../lib/stores/settings.svelte'
  import { listThemes, setTheme } from '../../themes/registry'

  const themes = listThemes()
  const copy = $derived(getCopy(settings.state.language))
</script>

<section class="theme-page">
  <header class="page-head">
    <p class="eyebrow">{copy.theme.eyebrow}</p>
    <h1>{copy.theme.title}</h1>
  </header>

  <div class="themes">
    {#each themes as theme (theme.id)}
      <button
        class="motion-press"
        type="button"
        class:active={settings.state.theme === theme.id}
        disabled={theme.status !== 'available'}
        onclick={async () => {
          await setTheme(theme.id)
          await settings.update('theme', theme.id)
        }}
      >
        <span class="swatch" style={`--preview:${theme.preview}`}></span>
        <strong>{theme.displayName}</strong>
        <em>{theme.status === 'available' ? copy.theme.available : copy.theme.comingSoon}</em>
      </button>
    {/each}
  </div>
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

  h1 {
    margin: 7px 0 0;
    font-family: var(--font-display);
    font-size: clamp(30px, 5vw, 54px);
  }

  .themes {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    border-bottom: 1px solid var(--color-border);
  }

  button {
    min-height: 170px;
    border: 0;
    border-right: 1px solid var(--color-border);
    border-bottom: 1px solid var(--color-border);
    cursor: pointer;
    display: grid;
    align-content: center;
    gap: 12px;
    justify-items: start;
    padding: 22px;
    text-align: left;
  }

  button:disabled {
    cursor: default;
    opacity: 0.45;
  }

  button.active,
  button:hover:not(:disabled) {
    background: var(--color-accent);
    color: var(--color-bg);
  }

  .swatch {
    width: 42px;
    height: 42px;
    background: var(--preview);
    border: var(--border-width) solid var(--color-fg);
    box-shadow: var(--shadow-hard-sm);
  }

  strong {
    font-family: var(--font-display);
    font-size: 22px;
  }

  em {
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 11px;
    font-style: normal;
    letter-spacing: 2px;
    text-transform: uppercase;
  }
</style>
