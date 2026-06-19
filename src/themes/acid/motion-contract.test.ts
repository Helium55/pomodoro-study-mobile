import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const root = resolve(import.meta.dirname, '../../..')

function read(path: string) {
  return readFileSync(resolve(root, path), 'utf8')
}

describe('acid motion contract', () => {
  it('keeps motion restrained and reusable through theme-level primitives', () => {
    const css = read('src/themes/acid/animations.css')

    expect(css).toContain('@keyframes acid-page-in')
    expect(css).toContain('@keyframes acid-edge-rest')
    expect(css).toContain('@keyframes acid-selected-pulse')
    expect(css).toContain('@keyframes acid-progress-tick')
    expect(css).not.toContain('translateX')
    expect(css).toContain('.motion-page')
    expect(css).toContain('.motion-press')
    expect(css).toContain('.motion-selected')
    expect(css).toContain('@media (prefers-reduced-motion: reduce)')
  })

  it('applies motion hooks to existing UI surfaces without changing the layout model', () => {
    const layout = read('src/routes/+layout.svelte')
    const focusPage = read('src/routes/focus/+page.svelte')
    const taskRow = read('src/lib/components/TaskRow.svelte')
    const timerDisplay = read('src/lib/components/TimerDisplay.svelte')
    const progressBar = read('src/lib/components/ProgressBar.svelte')

    expect(layout).toContain('class="content motion-page"')
    expect(focusPage).toContain('class="taskbar motion-edge"')
    expect(focusPage).toContain('class="primary motion-press"')
    expect(taskRow).toContain('class:selected')
    expect(taskRow).toContain('class:motion-selected={selected}')
    expect(timerDisplay).toContain('class:running={status ===')
    expect(progressBar).toContain('class="progress motion-progress"')
  })
})
