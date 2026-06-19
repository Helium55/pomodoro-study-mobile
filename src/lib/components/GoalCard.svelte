<script lang="ts">
  import { Archive, Target } from '@lucide/svelte'
  import { getCopy } from '../i18n'
  import { settings } from '../stores/settings.svelte'
  import type { Goal } from '../types'

  let { goal, onArchive }: { goal: Goal; onArchive: (goal: Goal) => void } = $props()
  const copy = $derived(getCopy(settings.state.language))
</script>

<article class="goal motion-edge">
  <Target size={18} />
  <div>
    <h3>{goal.title}</h3>
    {#if goal.description}
      <p>{goal.description}</p>
    {/if}
  </div>
  <button class="motion-press" type="button" onclick={() => onArchive(goal)} aria-label={copy.dialog.archiveGoal}>
    <Archive size={16} />
  </button>
</article>

<style>
  .goal {
    display: grid;
    grid-template-columns: 28px minmax(0, 1fr) 40px;
    align-items: center;
    min-height: 64px;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg-elevated);
    padding: 0 12px;
  }

  h3,
  p {
    margin: 0;
  }

  h3 {
    overflow: hidden;
    color: var(--color-fg);
    font-size: 13px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  p {
    overflow: hidden;
    color: var(--color-fg-muted);
    font-size: 11px;
    margin-top: 3px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  button {
    width: 34px;
    height: 34px;
    border: 1px solid var(--color-border);
    color: var(--color-fg-muted);
    cursor: pointer;
    display: grid;
    place-items: center;
  }

  button:hover {
    background: var(--color-accent);
    color: var(--color-bg);
  }
</style>
