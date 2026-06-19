<script lang="ts">
  import { Check, Play, Trash2 } from '@lucide/svelte'
  import { getCopy } from '../i18n'
  import { settings } from '../stores/settings.svelte'
  import type { Task } from '../types'
  import ProgressBar from './ProgressBar.svelte'

  let {
    task,
    selected = false,
    onSelect,
    onComplete,
    onDelete,
  }: {
    task: Task
    selected?: boolean
    onSelect: (task: Task) => void
    onComplete: (task: Task) => void
    onDelete: (task: Task) => void
  } = $props()

  const ratio = $derived(task.estimated_pomos > 0 ? task.completed_pomos / task.estimated_pomos : 0)
  const copy = $derived(getCopy(settings.state.language))
</script>

<article class="row motion-edge" class:selected class:motion-selected={selected}>
  <button
    class="square motion-press"
    type="button"
    onclick={() => onComplete(task)}
    aria-label={copy.dialog.completeTask}
  >
    <Check size={15} />
  </button>
  <button class="main motion-press" type="button" onclick={() => onSelect(task)}>
    <span class="title">{task.title}</span>
    <span class="sub">
      <ProgressBar value={ratio} total={Math.max(1, task.estimated_pomos)} />
      {task.completed_pomos} / {task.estimated_pomos}
    </span>
  </button>
  <button
    class="icon motion-press"
    type="button"
    onclick={() => onSelect(task)}
    aria-label={copy.dialog.selectTask}
  >
    <Play size={15} />
  </button>
  <button
    class="icon motion-press"
    type="button"
    onclick={() => onDelete(task)}
    aria-label={copy.dialog.deleteTask}
  >
    <Trash2 size={15} />
  </button>
</article>

<style>
  .row {
    display: grid;
    grid-template-columns: 34px minmax(0, 1fr) 36px 36px;
    min-height: 58px;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg);
  }

  .row.selected {
    background: var(--color-bg-elevated);
    border-left: 3px solid var(--color-accent);
  }

  button {
    border: 0;
    border-right: 1px solid var(--color-border);
    cursor: pointer;
  }

  .square,
  .icon {
    display: grid;
    place-items: center;
    color: var(--color-fg-muted);
  }

  .square:hover,
  .icon:hover {
    background: var(--color-accent);
    color: var(--color-bg);
  }

  .main {
    display: grid;
    align-content: center;
    gap: 5px;
    min-width: 0;
    padding: 8px 12px;
    text-align: left;
  }

  .title {
    overflow: hidden;
    color: var(--color-fg);
    font-size: 13px;
    font-weight: 800;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .sub {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 1px;
  }
</style>
