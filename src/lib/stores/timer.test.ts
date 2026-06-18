import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { settings } from './settings.svelte'
import { timer } from './timer.svelte'

interface TimerInternals {
  endsAtMs: number
  startMs: number
  tick: () => void
  reset: () => void
}

describe('timer absolute timekeeping', () => {
  beforeEach(() => {
    vi.useFakeTimers()
    vi.setSystemTime(new Date('2026-06-18T07:00:00.000Z'))
    settings.state.workSecs = 10
    timer.reset()
  })

  afterEach(() => {
    timer.reset()
    vi.useRealTimers()
  })

  it('sets an absolute end timestamp when focus starts', async () => {
    await timer.startFocus(null)

    expect((timer as unknown as TimerInternals).endsAtMs).toBe(Date.now() + 10_000)
  })

  it('recomputes remaining seconds from the absolute end timestamp', async () => {
    await timer.startFocus(null)
    const internals = timer as unknown as TimerInternals
    internals.startMs = Date.now() - 60_000
    internals.endsAtMs = Date.now() + 5_000

    internals.tick()

    expect(timer.remainingSecs).toBe(5)
  })
})
