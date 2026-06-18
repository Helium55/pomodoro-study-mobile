import '../styles/reset.css'
import './acid/theme.css'
import './acid/animations.css'
import './_base.css'
import { ipc } from '../lib/ipc'
import { getTheme } from './registry'

export async function bootstrapTheme() {
  const raw = await ipc.getSetting('theme')
  const id = raw ? (JSON.parse(raw) as string) : 'acid'
  document.documentElement.dataset.theme = getTheme(id)?.id ?? 'acid'
}
