<script lang="ts">
  import { X } from '@lucide/svelte'
  import { getCopy } from '../i18n'
  import { settings } from '../stores/settings.svelte'

  let {
    open,
    onCancel,
    onSubmit,
  }: { open: boolean; onCancel: () => void; onSubmit: (reason: string) => void } = $props()

  let reason = $state('')
  const copy = $derived(getCopy(settings.state.language))

  function submit() {
    onSubmit(reason)
    reason = ''
  }
</script>

{#if open}
  <div class="backdrop">
    <form
      class="dialog"
      onsubmit={(event) => {
        event.preventDefault()
        submit()
      }}
    >
      <header>
        <h2>{copy.dialog.interruptTitle}</h2>
        <button type="button" aria-label={copy.dialog.close} onclick={onCancel}>
          <X size={18} />
        </button>
      </header>
      <textarea bind:value={reason} placeholder={copy.dialog.reason}></textarea>
      <div class="actions">
        <button type="button" onclick={onCancel}>{copy.dialog.cancel}</button>
        <button class="primary" type="submit">{copy.dialog.save}</button>
      </div>
    </form>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 30;
    display: grid;
    place-items: center;
    background: var(--color-accent-low);
  }

  .dialog {
    width: min(460px, calc(100vw - 32px));
    border: var(--border-width) solid var(--color-fg);
    background: var(--color-bg);
    box-shadow: var(--shadow-hard-md);
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--color-border);
    padding: 12px 14px;
  }

  h2 {
    margin: 0;
    color: var(--color-accent);
    font-family: var(--font-mono);
    font-size: 12px;
    letter-spacing: 3px;
    text-transform: uppercase;
  }

  textarea {
    width: 100%;
    min-height: 120px;
    border: 0;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg-elevated);
    color: var(--color-fg);
    padding: 14px;
    resize: vertical;
  }

  .actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }

  button {
    min-height: 44px;
    border: 0;
    border-right: 1px solid var(--color-border);
    cursor: pointer;
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 900;
    letter-spacing: 2px;
  }

  .primary {
    background: var(--color-accent);
    color: var(--color-bg);
  }
</style>
