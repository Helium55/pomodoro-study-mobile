import { ipc } from '../ipc'
import type { Goal } from '../types'

class GoalsStore {
  items = $state<Goal[]>([])
  loaded = $state(false)

  get active() {
    return this.items.filter((goal) => goal.status === 'active')
  }

  async load(includeArchived = false) {
    this.items = await ipc.listGoals(includeArchived)
    this.loaded = true
  }

  async create(title: string, description = '') {
    const trimmed = title.trim()
    if (!trimmed) return
    await ipc.createGoal(trimmed, description.trim() || undefined)
    await this.load()
  }

  async archive(id: number) {
    await ipc.archiveGoal(id)
    await this.load()
  }

  async remove(id: number) {
    await ipc.deleteGoal(id)
    await this.load()
  }
}

export const goals = new GoalsStore()
