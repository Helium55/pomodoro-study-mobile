<script lang="ts">
  import { onMount } from 'svelte'
  import ProgressBar from '../../lib/components/ProgressBar.svelte'
  import { getCopy } from '../../lib/i18n'
  import { formatHours } from '../../lib/time'
  import { ipc } from '../../lib/ipc'
  import { settings } from '../../lib/stores/settings.svelte'
  import type { StatsSummary } from '../../lib/types'

  let stats = $state<StatsSummary | null>(null)
  const copy = $derived(getCopy(settings.state.language))

  onMount(() => {
    void load()
  })

  async function load() {
    stats = await ipc.getStats()
  }
</script>

<section class="stats-page">
  <header class="page-head">
    <p class="eyebrow">{copy.stats.eyebrow}</p>
    <h1>{copy.stats.title}</h1>
  </header>

  <div class="cards">
    <div>
      <span>{copy.stats.todayPomodoros}</span>
      <strong>{stats?.today.pomos ?? 0}</strong>
    </div>
    <div>
      <span>{copy.stats.todayFocus}</span>
      <strong>{formatHours(stats?.today.focus_secs ?? 0)}</strong>
    </div>
    <div>
      <span>{copy.stats.streak}</span>
      <strong>{stats?.streak_days ?? 0}</strong>
    </div>
  </div>

  <section class="panel">
    <h2>{copy.stats.last7Days}</h2>
    <div class="bars">
      {#each stats?.last_7_days ?? [] as day (day.date)}
        <div class="bar-row">
          <span>{day.date.slice(5)}</span>
          <ProgressBar value={Math.min(1, day.pomos / 8)} total={18} />
          <strong>{day.pomos}</strong>
        </div>
      {/each}
    </div>
  </section>

  <section class="grid">
    <div class="panel">
      <h2>{copy.stats.byGoal}</h2>
      {#each stats?.by_goal ?? [] as row (row.goal_id ?? row.goal_title)}
        <div class="line">
          <span>{row.goal_title}</span>
          <strong>{row.pomos} / {formatHours(row.focus_secs)}</strong>
        </div>
      {/each}
    </div>
    <div class="panel">
      <h2>{copy.stats.interruptions}</h2>
      {#each stats?.top_interrupts ?? [] as row (row.reason)}
        <div class="line">
          <span>{row.reason}</span>
          <strong>{row.count}</strong>
        </div>
      {/each}
    </div>
  </section>
</section>

<style>
  .page-head {
    border-bottom: 1px solid var(--color-border);
    padding: 22px clamp(18px, 4vw, 34px);
  }

  .eyebrow,
  .cards span {
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 3px;
    margin: 0;
    text-transform: uppercase;
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
    margin-bottom: 14px;
  }

  .cards {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    border-bottom: 1px solid var(--color-border);
  }

  .cards div {
    border-right: 1px solid var(--color-border);
    padding: 18px;
  }

  .cards strong {
    display: block;
    font-family: var(--font-display);
    font-size: clamp(32px, 5vw, 58px);
    line-height: 1;
    margin-top: 8px;
  }

  .panel {
    border-bottom: 1px solid var(--color-border);
    padding: 20px;
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }

  .grid .panel {
    border-right: 1px solid var(--color-border);
  }

  .bar-row,
  .line {
    display: grid;
    grid-template-columns: 82px minmax(0, 1fr) auto;
    gap: 14px;
    align-items: center;
    min-height: 34px;
    border-bottom: 1px solid var(--color-border);
    font-family: var(--font-mono);
    font-size: 12px;
  }

  .bar-row > *,
  .line > * {
    min-width: 0;
  }

  .line {
    grid-template-columns: minmax(0, 1fr) auto;
  }

  .line span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @media (width <= 760px) {
    .page-head {
      padding: 18px 16px;
    }

    .cards,
    .grid {
      grid-template-columns: 1fr;
    }

    .cards div,
    .panel {
      padding: 16px;
    }

    .bar-row {
      grid-template-columns: 58px minmax(0, 1fr) auto;
      gap: 10px;
    }
  }
</style>
