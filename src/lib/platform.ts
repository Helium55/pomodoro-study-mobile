export type PlatformOs = 'android' | 'desktop' | 'browser'
export type FormFactor = 'phone' | 'tablet' | 'desktop'

export interface PlatformInput {
  userAgent: string
  width: number
  hasTauriInternals: boolean
}

export interface PlatformProfile {
  os: PlatformOs
  formFactor: FormFactor
  isTauri: boolean
  isAndroid: boolean
  isMobileLayout: boolean
}

export function getPlatformProfile(input: PlatformInput): PlatformProfile {
  const isAndroid = /Android/i.test(input.userAgent)
  const isPhoneUa = /Mobile/i.test(input.userAgent)
  const isTauri = input.hasTauriInternals
  const os: PlatformOs = isAndroid ? 'android' : isTauri ? 'desktop' : 'browser'
  const formFactor: FormFactor = isAndroid
    ? !isPhoneUa && input.width >= 840
      ? 'tablet'
      : 'phone'
    : 'desktop'

  return {
    os,
    formFactor,
    isTauri,
    isAndroid,
    isMobileLayout: formFactor === 'phone',
  }
}

export function readPlatformProfile(): PlatformProfile {
  if (typeof window === 'undefined') {
    return getPlatformProfile({ userAgent: '', width: 1280, hasTauriInternals: false })
  }

  return getPlatformProfile({
    userAgent: window.navigator.userAgent,
    width: window.innerWidth,
    hasTauriInternals: '__TAURI_INTERNALS__' in window,
  })
}
