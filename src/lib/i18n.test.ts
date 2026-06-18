import { describe, expect, test } from 'vitest'
import { getCopy, normalizeLanguage } from './i18n'

describe('i18n', () => {
  test('defaults unsupported languages to Simplified Chinese', () => {
    expect(normalizeLanguage(undefined)).toBe('zh-CN')
    expect(normalizeLanguage('fr')).toBe('zh-CN')
    expect(getCopy('fr').settings.title).toBe('偏好设置')
  })

  test('returns English copy when English is selected', () => {
    expect(normalizeLanguage('en')).toBe('en')
    expect(getCopy('en').settings.language).toBe('Language')
  })
})
