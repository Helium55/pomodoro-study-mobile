<script lang="ts">
  import { onMount } from 'svelte'
  import { CircleStop, Pause, Play, RotateCcw, SkipForward } from '@lucide/svelte'
  import CompleteOverlay from '../../lib/components/CompleteOverlay.svelte'
  import InterruptDialog from '../../lib/components/InterruptDialog.svelte'
  import TimerDisplay from '../../lib/components/TimerDisplay.svelte'
  import { getCopy } from '../../lib/i18n'
  import { formatHours } from '../../lib/time'
  import { ipc } from '../../lib/ipc'
  import { settings } from '../../lib/stores/settings.svelte'
  import { tasks } from '../../lib/stores/tasks.svelte'
  import { timer } from '../../lib/stores/timer.svelte'
  import type { StatsSummary } from '../../lib/types'

  let interruptOpen = $state(false)
  let stats = $state<StatsSummary | null>(null)
  let selectedTaskValue = $derived(tasks.selectedTaskId)
  const copy = $derived(getCopy(settings.state.language))

  onMount(() => {
    void refresh()
  })

  async function refresh() {
    await settings.load()
    await tasks.load()
    stats = await ipc.getStats()
  }

  async function startOrResume() {
    if (timer.status === 'paused') {
      timer.resume()
      return
    }
    await timer.startFocus(tasks.selectedTaskId)
  }

  async function saveInterrupt(reason: string) {
    interruptOpen = false
    await timer.interrupt(reason)
    stats = await ipc.getStats()
  }
</script>

<section class="focus-page">
  <header class="top">
    <div>
      <p class="eyebrow">{copy.focus.eyebrow}</p>
      <h1>{copy.timer.phase[timer.phase]}</h1>
    </div>
    <div class="session">
      <span>{copy.focus.today}</span>
      <strong>{stats?.today.pomos ?? 0}</strong>
    </div>
  </header>

  <div class="taskbar motion-edge">
    <label for="task-select">{copy.focus.currentTask}</label>
    <select
      id="task-select"
      bind:value={selectedTaskValue}
      onchange={() => tasks.select(selectedTaskValue)}
    >
      <option value={null}>{copy.focus.noTask}</option>
      {#each tasks.active as task (task.id)}
        <option value={task.id}>{task.title}</option>
      {/each}
    </select>
  </div>

  <div class="timer-wrap">
    <TimerDisplay
      label={copy.timer.phase[timer.phase]}
      statusLabel={copy.timer.status[timer.status]}
      remainingSecs={timer.remainingSecs}
      progress={timer.progress}
      status={timer.status}
    />
  </div>

  <div class="controls" aria-label={copy.focus.controls}>
    <button class="primary motion-press" type="button" onclick={startOrResume}>
      <Play size={18} />
      <span>{timer.status === 'paused' ? copy.focus.resume : copy.focus.start}</span>
    </button>
    <button class="motion-press" type="button" onclick={() => timer.pause()} disabled={timer.status !== 'running'}>
      <Pause size={18} />
      <span>{copy.focus.pause}</span>
    </button>
    <button
      class="motion-press"
      type="button"
      onclick={() => (interruptOpen = true)}
      disabled={timer.phase !== 'focus'}
    >
      <CircleStop size={18} />
      <span>{copy.focus.interrupt}</span>
    </button>
    <button class="motion-press" type="button" onclick={() => timer.finishPhase()} disabled={timer.status === 'idle'}>
      <SkipForward size={18} />
      <span>{copy.focus.skip}</span>
    </button>
    <button class="motion-press" type="button" onclick={() => timer.reset()}>
      <RotateCcw size={18} />
      <span>{copy.focus.reset}</span>
    </button>
  </div>

  <footer class="metrics">
    <div class="motion-edge">
      <span>{copy.focus.focusTime}</span>
      <strong>{formatHours(stats?.today.focus_secs ?? 0)}</strong>
    </div>
    <div class="motion-edge">
      <span>{copy.focus.interrupts}</span>
      <strong>{stats?.today.interrupts ?? 0}</strong>
    </div>
    <div class="motion-edge">
      <span>{copy.focus.selected}</span>
      <strong>{tasks.selected?.title ?? copy.focus.noTask}</strong>
    </div>
  </footer>
</section>

<InterruptDialog
  open={interruptOpen}
  onCancel={() => (interruptOpen = false)}
  onSubmit={saveInterrupt}
/>
<CompleteOverlay show={timer.completeOverlay} onClose={() => (timer.completeOverlay = false)} />

<style>
  .focus-page {
    min-height: 100%;
    display: grid;
    grid-template-rows: auto auto 1fr auto auto;
  }

  .top {
    display: flex;
    justify-content: space-between;
    gap: 18px;
    border-bottom: 1px solid var(--color-border);
    padding: 22px clamp(18px, 4vw, 34px);
  }

  .eyebrow,
  label,
  .session span,
  .metrics span {
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 3px;
    text-transform: uppercase;
  }

  h1 {
    margin: 7px 0 0;
    font-family: var(--font-display);
    font-size: clamp(30px, 5vw, 54px);
    line-height: 0.95;
  }

  .session {
    min-width: 118px;
    border: var(--border-width) solid var(--color-accent);
    display: grid;
    place-items: center;
    padding: 8px;
  }

  .session strong {
    color: var(--color-accent);
    font-family: var(--font-display);
    font-size: 36px;
    line-height: 1;
  }

  .taskbar {
    display: grid;
    grid-template-columns: 150px minmax(0, 1fr);
    align-items: center;
    border-bottom: 1px solid var(--color-border);
    padding: 12px clamp(18px, 4vw, 34px);
  }

  select {
    min-width: 0;
    border: var(--border-width) solid var(--color-border);
    background: var(--color-bg-elevated);
    color: var(--color-fg);
    font-family: var(--font-mono);
    padding: 10px 12px;
  }

  .timer-wrap {
    display: grid;
    place-items: center;
    padding: clamp(28px, 7vw, 72px) 20px;
  }

  .controls {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    border-top: 1px solid var(--color-border);
    border-bottom: 1px solid var(--color-border);
  }

  .controls button {
    min-height: 58px;
    border: 0;
    border-right: 1px solid var(--color-border);
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

  .controls button:disabled {
    color: var(--color-fg-muted);
    cursor: default;
  }

  .controls button:hover:not(:disabled),
  .controls .primary {
    background: var(--color-accent);
    color: var(--color-bg);
  }

  .metrics {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .metrics div {
    min-width: 0;
    border-right: 1px solid var(--color-border);
    padding: 14px 18px;
  }

  .metrics strong {
    display: block;
    overflow: hidden;
    font-family: var(--font-mono);
    font-size: 14px;
    margin-top: 5px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @media (width <= 760px) {
    .focus-page {
      grid-template-rows: auto auto minmax(260px, 1fr) auto auto;
    }

    .top {
      padding: 16px;
    }

    .taskbar,
    .metrics {
      grid-template-columns: 1fr;
      gap: 8px;
    }

    .timer-wrap {
      padding: 28px 14px;
    }

    .controls {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .controls .primary {
      grid-column: 1 / -1;
    }

    .controls button {
      min-height: 56px;
      border-bottom: 1px solid var(--color-border);
      font-size: 10px;
      letter-spacing: 1px;
    }

    .metrics div {
      padding: 12px 16px;
    }
  }

  @media (orientation: landscape) and (width <= 900px) and (height <= 560px) {
    .focus-page {
      grid-template-columns: minmax(0, 1.15fr) minmax(260px, 0.85fr);
      grid-template-rows: auto auto 1fr;
      min-height: 100%;
    }

    .top {
      grid-column: 1 / -1;
      padding: 8px 14px;
    }

    h1 {
      font-size: 30px;
    }

    .session {
      min-width: 96px;
      padding: 6px;
    }

    .session strong {
      font-size: 28px;
    }

    .taskbar {
      grid-column: 2;
      grid-row: 2;
      border-left: 1px solid var(--color-border);
      padding: 10px 12px;
    }

    .timer-wrap {
      grid-column: 1;
      grid-row: 2 / 4;
      padding: 14px;
    }

    .controls {
      grid-column: 2;
      grid-row: 3;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      border-left: 1px solid var(--color-border);
    }

    .controls button {
      min-height: 44px;
    }

    .metrics {
      display: none;
    }
  }
</style>
