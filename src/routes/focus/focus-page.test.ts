// @vitest-environment jsdom
import { fireEvent, render, screen, waitFor } from '@testing-library/svelte'
import { beforeEach, describe, expect, it } from 'vitest'
import FocusPage from './+page.svelte'
import { settings } from '../../lib/stores/settings.svelte'
import { tasks } from '../../lib/stores/tasks.svelte'
import { timer } from '../../lib/stores/timer.svelte'

const memoryKey = 'pomodoro-study.memory-db'
const selectedKey = 'pomodoro-study.selected-task'

describe('focus page current task selection', () => {
  beforeEach(() => {
    localStorage.clear()
    timer.reset()
    settings.state.language = 'zh-CN'
    tasks.items = []
    tasks.selectedTaskId = 1
    tasks.loaded = false
    localStorage.setItem(selectedKey, '1')
    localStorage.setItem(
      memoryKey,
      JSON.stringify({
        goals: [],
        tasks: [
          {
            id: 1,
            goal_id: null,
            title: 'Reading chapter 1',
            estimated_pomos: 2,
            status: 'active',
            created_at: Date.now(),
            done_at: null,
            sort_order: 0,
            completed_pomos: 0,
          },
        ],
        pomodoros: [],
        interrupts: [],
        settings: {},
      }),
    )
  })

  it('shows a task that was selected before tasks finish loading', async () => {
    render(FocusPage)

    const select = screen.getByLabelText('当前任务') as HTMLSelectElement

    await waitFor(() => expect(select.options).toHaveLength(2))

    expect(select.value).toBe('1')
    expect(select.selectedOptions[0]?.textContent).toBe('Reading chapter 1')
  })

  it('shows the selected task after the first user change', async () => {
    tasks.selectedTaskId = null
    localStorage.removeItem(selectedKey)

    render(FocusPage)

    const select = screen.getByLabelText('当前任务') as HTMLSelectElement
    await waitFor(() => expect(select.options).toHaveLength(2))

    await fireEvent.change(select, { target: { value: '1' } })

    expect(select.value).toBe('1')
    expect(screen.getAllByText('Reading chapter 1')).toHaveLength(2)
  })
})
