<script lang="ts">
  import { formatSeconds } from '../time'
  import ProgressBar from './ProgressBar.svelte'

  let {
    label,
    statusLabel,
    remainingSecs,
    progress,
    status,
  }: {
    label: string
    statusLabel: string
    remainingSecs: number
    progress: number
    status: string
  } = $props()
</script>

<section class="timer" class:paused={status === 'paused'}>
  <div class="meta">{label} / {statusLabel}</div>
  <div class="digits">{formatSeconds(remainingSecs)}</div>
  <ProgressBar value={progress} total={14} />
</section>

<style>
  .timer {
    display: inline-grid;
    justify-items: center;
    gap: 10px;
    min-width: min(640px, 100%);
    border: var(--border-width) solid var(--color-fg);
    background: var(--color-bg);
    box-shadow: var(--shadow-hard-md);
    padding: 26px clamp(18px, 5vw, 42px);
  }

  .timer.paused {
    border-color: var(--color-accent);
  }

  .meta {
    color: var(--color-accent);
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 900;
    letter-spacing: 3px;
  }

  .digits {
    font-family: var(--font-display);
    font-size: clamp(58px, 13vw, 132px);
    font-variant-numeric: tabular-nums;
    font-weight: 900;
    line-height: 0.9;
  }

  @media (orientation: landscape) and (width <= 900px) and (height <= 560px) {
    .timer {
      gap: 7px;
      min-width: min(460px, 100%);
      padding: 18px clamp(14px, 3vw, 28px);
    }

    .digits {
      font-size: clamp(54px, 11vw, 92px);
    }
  }
</style>
