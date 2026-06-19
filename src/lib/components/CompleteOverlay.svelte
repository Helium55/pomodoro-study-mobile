<script lang="ts">
  import { getCopy } from '../i18n'
  import { settings } from '../stores/settings.svelte'

  let { show, onClose }: { show: boolean; onClose: () => void } = $props()

  const copy = $derived(getCopy(settings.state.language))
</script>

{#if show}
  <div class="overlay" role="status">
    <button class="motion-press" type="button" onclick={onClose}>{copy.dialog.completeOverlay}</button>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 40;
    display: grid;
    place-items: center;
    background: var(--color-accent-low);
    border: 12px solid var(--color-accent);
    animation: acid-scan 360ms var(--ease-snap);
  }

  button {
    border: var(--border-width) solid var(--color-bg);
    background: var(--color-accent);
    box-shadow: var(--shadow-hard-lg);
    color: var(--color-bg);
    cursor: pointer;
    font-family: var(--font-display);
    font-size: clamp(30px, 8vw, 92px);
    font-weight: 900;
    letter-spacing: 2px;
    padding: 18px 26px;
  }
</style>
