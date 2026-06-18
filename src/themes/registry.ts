import type { ThemeMeta } from '../lib/types'
import { ipc } from '../lib/ipc'
import { meta as acid } from './acid/meta'
import { meta as synthwave } from './synthwave/meta'

const themes = [acid, synthwave]

export function listThemes(): ThemeMeta[] {
  return themes
}

export function getTheme(id: string): ThemeMeta | undefined {
  return themes.find((theme) => theme.id === id)
}

export function getCurrentTheme(): string {
  if (typeof document === 'undefined') return 'acid'
  return document.documentElement.dataset.theme ?? 'acid'
}

export async function setTheme(id: string) {
  const theme = getTheme(id)
  if (!theme || theme.status !== 'available') return
  document.documentElement.dataset.theme = id
  await ipc.setSetting('theme', JSON.stringify(id))
}
