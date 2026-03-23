import { inject } from 'vue'

export const settingsContextKey = Symbol('launcher-settings-context')

export function useSettingsContext() {
  const context = inject(settingsContextKey)

  if (!context) {
    throw new Error('Launcher settings context is not available.')
  }

  return context
}
