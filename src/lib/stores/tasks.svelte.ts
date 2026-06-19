import { browser } from '$app/environment'
import { ipc } from '../ipc'
import type { Task } from '../types'

const selectedKey = 'pomodoro-study.selected-task'

function canUseLocalStorage() {
  return browser && typeof localStorage !== 'undefined'
}

class TasksStore {
  items = $state<Task[]>([])
  selectedTaskId = $state<number | null>(null)
  loaded = $state(false)

  constructor() {
    if (canUseLocalStorage()) {
      const raw = localStorage.getItem(selectedKey)
      this.selectedTaskId = raw ? Number(raw) : null
    }
  }

  get active() {
    return this.items.filter((task) => task.status === 'active')
  }

  get selected() {
    return this.items.find((task) => task.id === this.selectedTaskId) ?? null
  }

  async load(includeDone = false) {
    this.items = await ipc.listTasks(undefined, includeDone)
    if (this.selectedTaskId && !this.items.some((task) => task.id === this.selectedTaskId)) {
      this.select(null)
    }
    this.loaded = true
  }

  select(id: number | null) {
    this.selectedTaskId = id
    if (canUseLocalStorage()) {
      if (id) localStorage.setItem(selectedKey, String(id))
      else localStorage.removeItem(selectedKey)
    }
  }

  async create(goalId: number | null, title: string, estimatedPomos: number) {
    const trimmed = title.trim()
    if (!trimmed) return
    const task = await ipc.createTask(goalId, trimmed, estimatedPomos)
    this.select(task.id)
    await this.load()
  }

  async update(id: number, patch: { title?: string; goalId?: number | null; estimatedPomos?: number }) {
    await ipc.updateTask(id, patch)
    await this.load()
  }

  async complete(id: number) {
    await ipc.completeTask(id)
    await this.load()
  }

  async remove(id: number) {
    await ipc.deleteTask(id)
    if (this.selectedTaskId === id) this.select(null)
    await this.load()
  }
}

export const tasks = new TasksStore()
