<script setup>
import { Button, Card, Toggle } from 'omorphia'
import { i18n } from '@/main.js'
import { useSettingsContext } from './context'

const t = i18n.global.t

const { settings, themeStore } = useSettingsContext()

const themeCards = [
  {
    id: 'dark',
    titleKey: 'Settings.ThemeDarkTitle',
    descriptionKey: 'Settings.ThemeDarkDesc',
    swatches: ['#110711', '#34183b', '#f15bbf'],
  },
  {
    id: 'light',
    titleKey: 'Settings.ThemeLightTitle',
    descriptionKey: 'Settings.ThemeLightDesc',
    swatches: ['#f7e6ef', '#f1d5e2', '#e754a8'],
  },
  {
    id: 'oled',
    titleKey: 'Settings.ThemeOledTitle',
    descriptionKey: 'Settings.ThemeOledDesc',
    swatches: ['#040104', '#28122d', '#f768c6'],
  },
  {
    id: 'sunset',
    titleKey: 'Settings.ThemeSunsetTitle',
    descriptionKey: 'Settings.ThemeSunsetDesc',
    swatches: ['#1c0b12', '#592539', '#ff8d85'],
  },
  {
    id: 'lavender',
    titleKey: 'Settings.ThemeLavenderTitle',
    descriptionKey: 'Settings.ThemeLavenderDesc',
    swatches: ['#140c1d', '#3b2750', '#c88cff'],
  },
  {
    id: 'frost',
    titleKey: 'Settings.ThemeFrostTitle',
    descriptionKey: 'Settings.ThemeFrostDesc',
    swatches: ['#0f1724', '#253347', '#8fd9ff'],
  },
]

function selectTheme(themeId) {
  themeStore.setThemeState(themeId)
  settings.value.theme = themeStore.selectedTheme
}

function restoreDefaults() {
  selectTheme('dark')
  themeStore.advancedRendering = true
  settings.value.advanced_rendering = true
  themeStore.setCollapsedNavigation(false)
  settings.value.collapsed_navigation = false
  themeStore.restoreCustomizationDefaults()
}
</script>

<template>
  <div class="settings-panel">
    <Card class="settings-card">
      <div class="label">
        <h3>
          <span class="label__title size-card-header">{{ t('Settings.ThemesAndCustomization') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.ThemesSectionDescription') }}</span>
      </div>

      <div class="theme-grid">
        <button
          v-for="theme in themeCards"
          :key="theme.id"
          class="theme-card"
          :class="{ 'theme-card--active': settings.theme === theme.id }"
          :style="{
            '--theme-a': theme.swatches[0],
            '--theme-b': theme.swatches[1],
            '--theme-c': theme.swatches[2],
          }"
          @click="selectTheme(theme.id)"
        >
          <div class="theme-card__preview">
            <span class="theme-card__swatch theme-card__swatch--wide"></span>
            <div class="theme-card__row">
              <span class="theme-card__swatch"></span>
              <span class="theme-card__swatch"></span>
              <span class="theme-card__swatch"></span>
            </div>
          </div>
          <div class="theme-card__copy">
            <span class="theme-card__title">{{ t(theme.titleKey) }}</span>
            <span class="theme-card__description">{{ t(theme.descriptionKey) }}</span>
          </div>
        </button>
      </div>
    </Card>

    <Card class="settings-card">
      <div class="label">
        <h3>
          <span class="label__title size-card-header">{{ t('Settings.CustomizationControls') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.CustomizationSectionDescription') }}</span>
      </div>

      <div class="adjacent-input">
        <label for="advanced-rendering">
          <span class="label__title">{{ t('Settings.AdvancedRendering') }}</span>
          <span class="label__description">{{ t('Settings.EnablesAdvancedRendering') }}</span>
        </label>
        <Toggle
          id="advanced-rendering"
          :model-value="themeStore.advancedRendering"
          :checked="themeStore.advancedRendering"
          @update:model-value="
            (event) => {
              themeStore.advancedRendering = event
              settings.advanced_rendering = themeStore.advancedRendering
            }
          "
        />
      </div>

      <div class="adjacent-input">
        <label for="collapsed-navigation">
          <span class="label__title">{{ t('Settings.CompactSidebar') }}</span>
          <span class="label__description">{{ t('Settings.CompactSidebarDesc') }}</span>
        </label>
        <Toggle
          id="collapsed-navigation"
          :model-value="settings.collapsed_navigation"
          :checked="settings.collapsed_navigation"
          @update:model-value="
            (event) => {
              themeStore.setCollapsedNavigation(event)
              settings.collapsed_navigation = event
            }
          "
        />
      </div>

      <div class="adjacent-input">
        <label for="ambient-effects">
          <span class="label__title">{{ t('Settings.AmbientEffects') }}</span>
          <span class="label__description">{{ t('Settings.AmbientEffectsDesc') }}</span>
        </label>
        <Toggle
          id="ambient-effects"
          :model-value="themeStore.ambientEffects"
          :checked="themeStore.ambientEffects"
          @update:model-value="themeStore.setAmbientEffects"
        />
      </div>

      <div class="adjacent-input">
        <label for="reduced-motion">
          <span class="label__title">{{ t('Settings.ReducedMotion') }}</span>
          <span class="label__description">{{ t('Settings.ReducedMotionDesc') }}</span>
        </label>
        <Toggle
          id="reduced-motion"
          :model-value="themeStore.reducedMotion"
          :checked="themeStore.reducedMotion"
          @update:model-value="themeStore.setReducedMotion"
        />
      </div>

      <div class="controls-footer">
        <Button @click="restoreDefaults()">{{ t('Settings.ResetCustomizationDefaults') }}</Button>
      </div>
    </Card>
  </div>
</template>

<style scoped lang="scss">
.settings-panel {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.settings-card {
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.theme-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(15rem, 1fr));
  gap: 0.85rem;
}

.theme-card {
  appearance: none;
  border: 1px solid color-mix(in srgb, var(--color-button-bg) 72%, transparent 28%);
  background: color-mix(in srgb, var(--color-raised-bg) 85%, transparent 15%);
  border-radius: var(--radius-xl);
  padding: 0.9rem;
  text-align: left;
  color: inherit;
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
  cursor: pointer;
  transition: 0.15s ease-in-out;
}

.theme-card:hover,
.theme-card:focus-visible {
  transform: translateY(-1px);
  border-color: color-mix(in srgb, var(--color-brand) 38%, transparent 62%);
  box-shadow: var(--shadow-floating);
}

.theme-card--active {
  border-color: color-mix(in srgb, var(--color-brand) 50%, transparent 50%);
  box-shadow: 0 0 0 1px color-mix(in srgb, var(--color-brand) 25%, transparent 75%), var(--shadow-floating);
}

.theme-card__preview {
  display: flex;
  flex-direction: column;
  gap: 0.55rem;
  padding: 0.75rem;
  border-radius: var(--radius-lg);
  background: linear-gradient(135deg, var(--theme-a) 0%, var(--theme-b) 100%);
  min-height: 5.8rem;
}

.theme-card__row {
  display: grid;
  grid-template-columns: 1.3fr 1fr 0.8fr;
  gap: 0.45rem;
}

.theme-card__swatch {
  display: block;
  height: 1.45rem;
  border-radius: var(--radius-md);
  background: color-mix(in srgb, var(--theme-c) 80%, white 20%);
  opacity: 0.94;
}

.theme-card__swatch--wide {
  height: 1.8rem;
  background: linear-gradient(135deg, color-mix(in srgb, var(--theme-c) 82%, white 18%) 0%, transparent 100%);
}

.theme-card__copy {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.theme-card__title {
  font-weight: 700;
  color: var(--color-contrast);
}

.theme-card__description {
  font-size: 0.92rem;
  line-height: 1.35;
  opacity: 0.78;
}

.controls-footer {
  display: flex;
  justify-content: flex-end;
}
</style>
