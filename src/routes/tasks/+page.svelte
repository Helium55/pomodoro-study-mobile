<script lang="ts">
  import { onMount } from 'svelte'
  import { Plus } from '@lucide/svelte'
  import GoalCard from '../../lib/components/GoalCard.svelte'
  import TaskRow from '../../lib/components/TaskRow.svelte'
  import { getCopy } from '../../lib/i18n'
  import { goals } from '../../lib/stores/goals.svelte'
  import { settings } from '../../lib/stores/settings.svelte'
  import { tasks } from '../../lib/stores/tasks.svelte'

  let goalTitle = $state('')
  let goalDescription = $state('')
  let taskTitle = $state('')
  let taskGoal = $state('')
  let estimatedPomos = $state(1)
  const copy = $derived(getCopy(settings.state.language))

  onMount(() => {
    void goals.load()
    void tasks.load()
  })

  async function addGoal() {
    await goals.create(goalTitle, goalDescription)
    goalTitle = ''
    goalDescription = ''
  }

  async function addTask() {
    await tasks.create(taskGoal ? Number(taskGoal) : null, taskTitle, estimatedPomos)
    taskTitle = ''
    estimatedPomos = 1
  }
</script>

<section class="tasks-page">
  <header class="page-head">
    <p class="eyebrow">{copy.tasks.eyebrow}</p>
    <h1>{copy.tasks.title}</h1>
  </header>

  <div class="columns">
    <section class="pane motion-edge">
      <header class="pane-head">
        <h2>{copy.tasks.goals}</h2>
      </header>
      <form
        class="form"
        onsubmit={(event) => {
          event.preventDefault()
          void addGoal()
        }}
      >
        <input bind:value={goalTitle} placeholder={copy.tasks.goalTitle} />
        <input bind:value={goalDescription} placeholder={copy.tasks.description} />
        <button class="motion-press" type="submit" aria-label={copy.tasks.addGoal}>
          <Plus size={17} />
          <span>{copy.tasks.addGoal}</span>
        </button>
      </form>
      <div class="list">
        {#each goals.active as goal (goal.id)}
          <GoalCard {goal} onArchive={(item) => goals.archive(item.id)} />
        {/each}
      </div>
    </section>

    <section class="pane motion-edge">
      <header class="pane-head">
        <h2>{copy.tasks.tasks}</h2>
      </header>
      <form
        class="form task-form"
        onsubmit={(event) => {
          event.preventDefault()
          void addTask()
        }}
      >
        <input bind:value={taskTitle} placeholder={copy.tasks.taskTitle} />
        <select bind:value={taskGoal}>
          <option value="">{copy.tasks.noGoal}</option>
          {#each goals.active as goal (goal.id)}
            <option value={goal.id}>{goal.title}</option>
          {/each}
        </select>
        <input bind:value={estimatedPomos} type="number" min="1" max="24" />
        <button class="motion-press" type="submit" aria-label={copy.tasks.addTask}>
          <Plus size={17} />
          <span>{copy.tasks.addTask}</span>
        </button>
      </form>
      <div class="list">
        {#each tasks.active as task (task.id)}
          <TaskRow
            {task}
            selected={tasks.selectedTaskId === task.id}
            onSelect={(item) => tasks.select(item.id)}
            onComplete={(item) => tasks.complete(item.id)}
            onDelete={(item) => tasks.remove(item.id)}
          />
        {/each}
      </div>
    </section>
  </div>
</section>

<style>
  .tasks-page {
    min-height: 100%;
  }

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
    font-size: 18px;
    letter-spacing: 1px;
  }

  .columns {
    display: grid;
    grid-template-columns: minmax(280px, 0.9fr) minmax(320px, 1.1fr);
    min-height: calc(100vh - 117px);
  }

  .pane {
    border-right: 1px solid var(--color-border);
  }

  .pane-head {
    border-bottom: 1px solid var(--color-border);
    padding: 14px 18px;
  }

  .form {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    border-bottom: 1px solid var(--color-border);
  }

  .task-form {
    grid-template-columns: minmax(0, 1fr) 160px 80px 88px;
  }

  input,
  select {
    min-width: 0;
    min-height: 44px;
    border: 0;
    border-right: 1px solid var(--color-border);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg-elevated);
    color: var(--color-fg);
    font-family: var(--font-mono);
    padding: 12px;
  }

  button {
    min-height: 44px;
    border: 0;
    background: var(--color-accent);
    color: var(--color-bg);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 900;
    letter-spacing: 2px;
  }

  @media (width <= 980px) {
    .columns,
    .task-form {
      grid-template-columns: 1fr;
    }
  }

  @media (width <= 760px) {
    .page-head {
      padding: 18px 16px;
    }

    .columns {
      min-height: auto;
    }

    .pane {
      border-right: 0;
      border-bottom: 1px solid var(--color-border);
    }

    .form {
      gap: 0;
    }
  }
</style>
