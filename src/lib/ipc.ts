import { invoke } from '@tauri-apps/api/core'
import type { Goal, Pomodoro, StatsSummary, Task } from './types'
import { localDate } from './time'

type CommandArgs = Record<string, unknown>

interface MemoryDb {
  goals: Goal[]
  tasks: Task[]
  pomodoros: Pomodoro[]
  interrupts: { id: number; pomodoro_id: number; reason: string | null; occurred_at: number }[]
  settings: Record<string, string>
}

const defaults: Record<string, string> = {
  'timer.work_secs': '1500',
  'timer.break_secs': '300',
  'timer.long_break_secs': '900',
  'timer.long_break_every': '4',
  'timer.auto_continue': 'false',
  language: '"zh-CN"',
  'notify.system': 'true',
  'notify.sound': 'true',
  'notify.sound_file': '"ding.wav"',
  'notify.vibration': 'true',
  'notify.foreground': 'true',
  'notify.fullscreen': 'true',
  'notify.taskbar': 'true',
  theme: '"acid"',
}

const storageKey = 'pomodoro-study.memory-db'

function isTauriRuntime() {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

function nowMs() {
  return Date.now()
}

function loadMemory(): MemoryDb {
  if (typeof localStorage === 'undefined') {
    return { goals: [], tasks: [], pomodoros: [], interrupts: [], settings: { ...defaults } }
  }
  const raw = localStorage.getItem(storageKey)
  if (!raw) {
    return { goals: [], tasks: [], pomodoros: [], interrupts: [], settings: { ...defaults } }
  }
  const parsed = JSON.parse(raw) as MemoryDb
  const settings = { ...defaults, ...parsed.settings }
  if (settings['notify.sound_file'] === '"ding.mp3"') {
    settings['notify.sound_file'] = defaults['notify.sound_file']
  }
  return { ...parsed, settings }
}

function saveMemory(db: MemoryDb) {
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem(storageKey, JSON.stringify(db))
  }
}

function nextId(items: { id: number }[]) {
  return items.reduce((max, item) => Math.max(max, item.id), 0) + 1
}

async function call<T>(command: string, args: CommandArgs = {}): Promise<T> {
  if (isTauriRuntime()) {
    return invoke<T>(command, args)
  }
  return memoryInvoke<T>(command, args)
}

async function memoryInvoke<T>(command: string, args: CommandArgs): Promise<T> {
  const db = loadMemory()
  const save = () => saveMemory(db)

  switch (command) {
    case 'create_goal': {
      const goal: Goal = {
        id: nextId(db.goals),
        title: String(args.title),
        description: (args.description as string | undefined) ?? null,
        color: (args.color as string | undefined) ?? null,
        status: 'active',
        created_at: nowMs(),
        archived_at: null,
      }
      db.goals.push(goal)
      save()
      return goal as T
    }
    case 'list_goals': {
      const includeArchived = Boolean(args.includeArchived)
      return db.goals
        .filter((goal) => includeArchived || goal.status === 'active')
        .sort((a, b) => b.created_at - a.created_at) as T
    }
    case 'update_goal': {
      const goal = db.goals.find((item) => item.id === Number(args.id))
      if (goal) {
        if (args.title !== undefined) goal.title = String(args.title)
        if (args.description !== undefined) goal.description = String(args.description)
        if (args.color !== undefined) goal.color = String(args.color)
        save()
      }
      return undefined as T
    }
    case 'archive_goal': {
      const goal = db.goals.find((item) => item.id === Number(args.id))
      if (goal) {
        goal.status = 'archived'
        goal.archived_at = nowMs()
        save()
      }
      return undefined as T
    }
    case 'delete_goal': {
      db.goals = db.goals.filter((goal) => goal.id !== Number(args.id))
      db.tasks = db.tasks.map((task) =>
        task.goal_id === Number(args.id) ? { ...task, goal_id: null } : task,
      )
      save()
      return undefined as T
    }
    case 'create_task': {
      const goalId = (args.goalId as number | null | undefined) ?? null
      const task: Task = {
        id: nextId(db.tasks),
        goal_id: goalId,
        title: String(args.title),
        estimated_pomos: Number(args.estimatedPomos ?? 1),
        status: 'active',
        created_at: nowMs(),
        done_at: null,
        sort_order: db.tasks.filter((item) => item.goal_id === goalId).length,
        completed_pomos: 0,
      }
      db.tasks.push(task)
      save()
      return task as T
    }
    case 'list_tasks': {
      const goalId = args.goalId as number | null | undefined
      const includeDone = Boolean(args.includeDone)
      return db.tasks
        .filter((task) => goalId === undefined || task.goal_id === goalId)
        .filter((task) => includeDone || task.status === 'active')
        .map((task) => ({
          ...task,
          completed_pomos: db.pomodoros.filter(
            (pomo) => pomo.task_id === task.id && pomo.status === 'completed',
          ).length,
        }))
        .sort((a, b) => a.sort_order - b.sort_order) as T
    }
    case 'update_task': {
      const task = db.tasks.find((item) => item.id === Number(args.id))
      if (task) {
        if (args.title !== undefined) task.title = String(args.title)
        if (args.goalId !== undefined) task.goal_id = Number(args.goalId)
        if (args.estimatedPomos !== undefined) task.estimated_pomos = Number(args.estimatedPomos)
        save()
      }
      return undefined as T
    }
    case 'complete_task': {
      const task = db.tasks.find((item) => item.id === Number(args.id))
      if (task) {
        task.status = 'done'
        task.done_at = nowMs()
        save()
      }
      return undefined as T
    }
    case 'delete_task': {
      db.tasks = db.tasks.filter((task) => task.id !== Number(args.id))
      save()
      return undefined as T
    }
    case 'reorder_tasks': {
      const orderedIds = args.orderedIds as number[]
      orderedIds.forEach((id, index) => {
        const task = db.tasks.find((item) => item.id === id)
        if (task) task.sort_order = index
      })
      save()
      return undefined as T
    }
    case 'start_pomodoro': {
      const taskId = (args.taskId as number | null | undefined) ?? null
      const task = taskId ? db.tasks.find((item) => item.id === taskId) : undefined
      const pomo: Pomodoro = {
        id: nextId(db.pomodoros),
        task_id: taskId,
        goal_id: task?.goal_id ?? null,
        started_at: nowMs(),
        ended_at: null,
        planned_secs: Number(args.plannedSecs),
        actual_secs: null,
        status: 'in_progress',
        date_local: localDate(),
      }
      db.pomodoros.push(pomo)
      save()
      return pomo as T
    }
    case 'complete_pomodoro': {
      const pomo = db.pomodoros.find((item) => item.id === Number(args.id))
      if (pomo) {
        pomo.status = 'completed'
        pomo.ended_at = nowMs()
        pomo.actual_secs = Number(args.actualSecs)
        save()
      }
      return undefined as T
    }
    case 'interrupt_pomodoro': {
      const pomo = db.pomodoros.find((item) => item.id === Number(args.id))
      if (pomo) {
        pomo.status = args.abandoned ? 'abandoned' : 'interrupted'
        pomo.ended_at = nowMs()
        pomo.actual_secs = Number(args.actualSecs)
        db.interrupts.push({
          id: nextId(db.interrupts),
          pomodoro_id: pomo.id,
          reason: (args.reason as string | undefined) ?? null,
          occurred_at: nowMs(),
        })
        save()
      }
      return undefined as T
    }
    case 'list_pomodoros_today':
      return db.pomodoros.filter((pomo) => pomo.date_local === localDate()) as T
    case 'get_setting':
      return (db.settings[String(args.key)] ?? null) as T
    case 'set_setting':
      db.settings[String(args.key)] = String(args.value)
      save()
      return undefined as T
    case 'get_all_settings':
      return db.settings as T
    case 'get_stats':
      return buildMemoryStats(db) as T
    case 'export_data':
      return JSON.stringify(db, null, 2) as T
    case 'import_data': {
      const imported = JSON.parse(String(args.jsonText)) as MemoryDb
      saveMemory({ ...imported, settings: { ...defaults, ...imported.settings } })
      return undefined as T
    }
    case 'reset_data':
      saveMemory({ goals: [], tasks: [], pomodoros: [], interrupts: [], settings: { ...defaults } })
      return undefined as T
    case 'notify_vibration':
    case 'set_foreground_timer':
    case 'clear_foreground_timer':
      return undefined as T
    default:
      return undefined as T
  }
}

function buildMemoryStats(db: MemoryDb): StatsSummary {
  const today = localDate()
  const completed = db.pomodoros.filter((pomo) => pomo.status === 'completed')
  const todayCompleted = completed.filter((pomo) => pomo.date_local === today)
  const interruptsToday = db.interrupts.filter((interrupt) => {
    const pomo = db.pomodoros.find((item) => item.id === interrupt.pomodoro_id)
    return pomo?.date_local === today
  })

  const last_7_days = Array.from({ length: 7 }, (_, index) => {
    const date = new Date()
    date.setDate(date.getDate() - (6 - index))
    const label = localDate(date)
    const day = completed.filter((pomo) => pomo.date_local === label)
    return {
      date: label,
      pomos: day.length,
      focus_secs: day.reduce((sum, pomo) => sum + (pomo.actual_secs ?? 0), 0),
    }
  })

  const by_goal = db.goals.map((goal) => {
    const rows = completed.filter((pomo) => pomo.goal_id === goal.id)
    return {
      goal_id: goal.id,
      goal_title: goal.title,
      pomos: rows.length,
      focus_secs: rows.reduce((sum, pomo) => sum + (pomo.actual_secs ?? 0), 0),
    }
  })

  const reasonCounts = db.interrupts.reduce<Record<string, number>>((acc, interrupt) => {
    const key = interrupt.reason?.trim() || 'Unspecified'
    acc[key] = (acc[key] ?? 0) + 1
    return acc
  }, {})

  return {
    today: {
      pomos: todayCompleted.length,
      focus_secs: todayCompleted.reduce((sum, pomo) => sum + (pomo.actual_secs ?? 0), 0),
      interrupts: interruptsToday.length,
    },
    total: {
      pomos: completed.length,
      focus_secs: completed.reduce((sum, pomo) => sum + (pomo.actual_secs ?? 0), 0),
      interrupts: db.interrupts.length,
    },
    streak_days: last_7_days.filter((day) => day.pomos > 0).length,
    last_7_days,
    by_goal: by_goal.filter((row) => row.pomos > 0),
    top_interrupts: Object.entries(reasonCounts)
      .map(([reason, count]) => ({ reason, count }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 5),
  }
}

export const ipc = {
  createGoal: (title: string, description?: string, color?: string) =>
    call<Goal>('create_goal', { title, description, color }),
  listGoals: (includeArchived = false) => call<Goal[]>('list_goals', { includeArchived }),
  updateGoal: (id: number, patch: { title?: string; description?: string; color?: string }) =>
    call<void>('update_goal', { id, ...patch }),
  archiveGoal: (id: number) => call<void>('archive_goal', { id }),
  deleteGoal: (id: number) => call<void>('delete_goal', { id }),
  createTask: (goalId: number | null, title: string, estimatedPomos = 1) =>
    call<Task>('create_task', { goalId, title, estimatedPomos }),
  listTasks: (goalId?: number | null, includeDone = false) =>
    call<Task[]>('list_tasks', { goalId, includeDone }),
  updateTask: (
    id: number,
    patch: { title?: string; goalId?: number | null; estimatedPomos?: number },
  ) => call<void>('update_task', { id, ...patch }),
  completeTask: (id: number) => call<void>('complete_task', { id }),
  deleteTask: (id: number) => call<void>('delete_task', { id }),
  reorderTasks: (orderedIds: number[]) => call<void>('reorder_tasks', { orderedIds }),
  startPomodoro: (taskId: number | null, plannedSecs: number) =>
    call<Pomodoro>('start_pomodoro', { taskId, plannedSecs }),
  completePomodoro: (id: number, actualSecs: number) =>
    call<void>('complete_pomodoro', { id, actualSecs }),
  interruptPomodoro: (id: number, reason: string, actualSecs: number, abandoned: boolean) =>
    call<void>('interrupt_pomodoro', { id, reason, actualSecs, abandoned }),
  listPomodorosToday: () => call<Pomodoro[]>('list_pomodoros_today'),
  getSetting: (key: string) => call<string | null>('get_setting', { key }),
  setSetting: (key: string, value: string) => call<void>('set_setting', { key, value }),
  getAllSettings: () => call<Record<string, string>>('get_all_settings'),
  getStats: () => call<StatsSummary>('get_stats'),
  notifySystem: (title: string, body: string) => call<void>('notify_system', { title, body }),
  notifySound: (soundFile: string) => call<void>('notify_sound', { soundFile }),
  notifyVibration: () => call<void>('notify_vibration'),
  setForegroundTimer: (args: { phase: string; title: string; body: string; endsAtMs: number }) =>
    call<void>('set_foreground_timer', args),
  clearForegroundTimer: () => call<void>('clear_foreground_timer'),
  notifyFocusWindow: () => call<void>('notify_focus_window'),
  notifyTaskbarFlash: () => call<void>('notify_taskbar_flash'),
  exportData: () => call<string>('export_data'),
  importData: (jsonText: string) => call<void>('import_data', { jsonText }),
  resetData: () => call<void>('reset_data'),
}
