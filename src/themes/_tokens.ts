export const TOKEN_KEYS = [
  'color-bg',
  'color-bg-elevated',
  'color-fg',
  'color-fg-muted',
  'color-accent',
  'color-accent-low',
  'color-border',
  'color-success',
  'color-danger',
  'font-display',
  'font-body',
  'font-mono',
  'radius',
  'border-width',
  'shadow-hard-sm',
  'shadow-hard-md',
  'shadow-hard-lg',
  'ease-snap',
  'ease-smooth',
  'duration-fast',
  'duration-base'
] as const

export type TokenKey = (typeof TOKEN_KEYS)[number]
