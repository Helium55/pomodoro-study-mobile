import { describe, expect, it } from 'vitest'
import { ipc } from './ipc'

describe('ipc mobile reminder fallbacks', () => {
  it('resolves vibration and foreground timer calls outside Tauri', async () => {
    await expect(ipc.notifyVibration()).resolves.toBeUndefined()
    await expect(
      ipc.setForegroundTimer({
        phase: 'focus',
        title: 'FOCUS',
        body: 'Current task',
        endsAtMs: Date.now() + 1500,
      }),
    ).resolves.toBeUndefined()
    await expect(ipc.clearForegroundTimer()).resolves.toBeUndefined()
  })
})
