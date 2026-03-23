import { defineStore } from 'pinia'

const customizationStorageKey = 'vio.launcher.customization'

const defaultCustomizations = {
  ambientEffects: true,
  reducedMotion: false,
}

export const useTheming = defineStore('themeStore', {
  state: () => ({
    themeOptions: ['dark', 'light', 'oled', 'sunset', 'lavender', 'frost'],
    advancedRendering: true,
    selectedTheme: 'dark',
    collapsedNavigation: false,
    ambientEffects: defaultCustomizations.ambientEffects,
    reducedMotion: defaultCustomizations.reducedMotion,
  }),
  actions: {
    setThemeState(newTheme) {
      const normalizedTheme = `${newTheme}`.toLowerCase()

      if (this.themeOptions.includes(normalizedTheme)) this.selectedTheme = normalizedTheme
      else console.warn('Selected theme is not present. Check themeOptions.')

      this.setThemeClass()
    },
    setThemeClass() {
      const html = document.getElementsByTagName('html')[0]

      for (const theme of this.themeOptions) {
        html.classList.remove(`${theme}-mode`)
      }
      html.classList.add(`${this.selectedTheme}-mode`)

      this.applyCustomizationState()
    },
    setCollapsedNavigation(collapsed) {
      this.collapsedNavigation = collapsed
    },
    setAmbientEffects(enabled) {
      this.ambientEffects = enabled
      this.persistCustomizationState()
      this.applyCustomizationState()
    },
    setReducedMotion(enabled) {
      this.reducedMotion = enabled
      this.persistCustomizationState()
      this.applyCustomizationState()
    },
    restoreCustomizationDefaults() {
      this.ambientEffects = defaultCustomizations.ambientEffects
      this.reducedMotion = defaultCustomizations.reducedMotion
      this.persistCustomizationState()
      this.applyCustomizationState()
    },
    loadCustomizationState() {
      try {
        const rawState = window.localStorage.getItem(customizationStorageKey)
        if (!rawState) {
          this.applyCustomizationState()
          return
        }

        const parsedState = JSON.parse(rawState)

        this.ambientEffects = parsedState.ambientEffects ?? defaultCustomizations.ambientEffects
        this.reducedMotion = parsedState.reducedMotion ?? defaultCustomizations.reducedMotion
      } catch (error) {
        console.warn('Failed to load launcher customization state.', error)
        this.ambientEffects = defaultCustomizations.ambientEffects
        this.reducedMotion = defaultCustomizations.reducedMotion
      }

      this.applyCustomizationState()
    },
    persistCustomizationState() {
      window.localStorage.setItem(
        customizationStorageKey,
        JSON.stringify({
          ambientEffects: this.ambientEffects,
          reducedMotion: this.reducedMotion,
        }),
      )
    },
    applyCustomizationState() {
      const html = document.getElementsByTagName('html')[0]

      html.classList.toggle('ambient-effects-off', !this.ambientEffects)
      html.classList.toggle('reduced-motion', this.reducedMotion)
    },
  },
})
