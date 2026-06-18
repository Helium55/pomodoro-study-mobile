import { describe, expect, test } from 'vitest'
import { formatHours, formatSeconds, localDate, progressBlocks } from './time'

describe('time utilities', () => {
  test('formats seconds as mm:ss', () => {
    expect(formatSeconds(1500)).toBe('25:00')
    expect(formatSeconds(65)).toBe('01:05')
    expect(formatSeconds(-5)).toBe('00:00')
  })

  test('formats accumulated focus time as hours and minutes', () => {
    expect(formatHours(0)).toBe('0h 00m')
    expect(formatHours(3660)).toBe('1h 01m')
  })

  test('renders character progress blocks', () => {
    expect(progressBlocks(0, 4)).toBe('....')
    expect(progressBlocks(0.5, 4)).toBe('||..')
    expect(progressBlocks(1, 4)).toBe('||||')
  })

  test('returns a local YYYY-MM-DD date', () => {
    expect(localDate(new Date(2026, 5, 7))).toBe('2026-06-07')
  })
})
