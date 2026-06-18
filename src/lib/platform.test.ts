import { describe, expect, it } from 'vitest'
import { getPlatformProfile } from './platform'

describe('getPlatformProfile', () => {
  it('detects Android phones from the user agent', () => {
    const profile = getPlatformProfile({
      userAgent: 'Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36',
      width: 390,
      hasTauriInternals: true,
    })

    expect(profile.os).toBe('android')
    expect(profile.formFactor).toBe('phone')
    expect(profile.isMobileLayout).toBe(true)
  })

  it('detects Android tablets from width', () => {
    const profile = getPlatformProfile({
      userAgent: 'Mozilla/5.0 (Linux; Android 14; Tablet) AppleWebKit/537.36',
      width: 1100,
      hasTauriInternals: true,
    })

    expect(profile.os).toBe('android')
    expect(profile.formFactor).toBe('tablet')
    expect(profile.isMobileLayout).toBe(false)
  })

  it('keeps Android phones as phones in landscape when the user agent says mobile', () => {
    const profile = getPlatformProfile({
      userAgent: 'Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 Mobile',
      width: 844,
      hasTauriInternals: true,
    })

    expect(profile.os).toBe('android')
    expect(profile.formFactor).toBe('phone')
    expect(profile.isMobileLayout).toBe(true)
  })

  it('keeps desktop as desktop', () => {
    const profile = getPlatformProfile({
      userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)',
      width: 1280,
      hasTauriInternals: true,
    })

    expect(profile.os).toBe('desktop')
    expect(profile.formFactor).toBe('desktop')
    expect(profile.isMobileLayout).toBe(false)
  })
})
