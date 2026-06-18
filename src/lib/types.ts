export interface Goal {
  id: number
  title: string
  description: string | null
  color: string | null
  status: string
  created_at: number
  archived_at: number | null
}

export interface Task {
  id: number
  goal_id: number | null
  title: string
  estimated_pomos: number
  status: string
  created_at: number
  done_at: number | null
  sort_order: number
  completed_pomos: number
}

export interface Pomodoro {
  id: number
  task_id: number | null
  goal_id: number | null
  started_at: number
  ended_at: number | null
  planned_secs: number
  actual_secs: number | null
  status: string
  date_local: string
}

export interface StatBlock {
  pomos: number
  focus_secs: number
  interrupts: number
}

export interface DayStat {
  date: string
  pomos: number
  focus_secs: number
}

export interface GoalStat {
  goal_id: number | null
  goal_title: string
  pomos: number
  focus_secs: number
}

export interface InterruptStat {
  reason: string
  count: number
}

export interface StatsSummary {
  today: StatBlock
  total: StatBlock
  streak_days: number
  last_7_days: DayStat[]
  by_goal: GoalStat[]
  top_interrupts: InterruptStat[]
}

export interface ThemeMeta {
  id: string
  displayName: string
  preview: string
  status: 'available' | 'coming-soon'
}

export type Language = 'zh-CN' | 'en'

export interface SettingsState {
  language: Language
  workSecs: number
  breakSecs: number
  longBreakSecs: number
  longBreakEvery: number
  autoContinue: boolean
  notifySystem: boolean
  notifySound: boolean
  notifySoundFile: string
  notifyVibration: boolean
  notifyForeground: boolean
  notifyFullscreen: boolean
  notifyTaskbar: boolean
  theme: string
}
