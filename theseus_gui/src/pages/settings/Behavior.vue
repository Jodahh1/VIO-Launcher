<script setup>
import { Card, DropdownSelect, Toggle } from 'omorphia'
import { i18n } from '@/main.js'
import { useSettingsContext } from './context'

const t = i18n.global.t

const { settings, pageOptions, languageStore, isMacOS } = useSettingsContext()
</script>

<template>
  <div class="settings-panel">
    <Card class="settings-card">
      <div class="label">
        <h3>
          <span class="label__title size-card-header">{{ t('Settings.LauncherBehavior') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.BehaviorSectionDescription') }}</span>
      </div>

      <div class="adjacent-input">
        <label for="language">
          <span class="label__title">{{ t('Settings.Language') }}</span>
          <span class="label__description">{{ t('Settings.ChangeTheGlobalLauncherLanguages') }}</span>
        </label>
        <DropdownSelect
          id="language"
          name="Language dropdown"
          :options="languageStore.languageOptions"
          :default-value="settings.language"
          :model-value="settings.language"
          class="language-dropdown"
          @change="
            (event) => {
              languageStore.setLanguageState(event.option.toLowerCase())
              settings.language = languageStore.selectedLanguage
            }
          "
        />
      </div>

      <div class="adjacent-input">
        <label for="opening-page">
          <span class="label__title">{{ t('Settings.DefaultLandingPage') }}</span>
          <span class="label__description">{{ t('Settings.ChangeThePageToWhichTheLauncherOpensOn') }}</span>
        </label>
        <DropdownSelect
          id="opening-page"
          name="Opening page dropdown"
          :options="pageOptions"
          :default-value="settings.default_page"
          :model-value="settings.default_page"
          class="opening-page"
          @change="
            (event) => {
              settings.default_page = event.option
            }
          "
        />
      </div>

      <div class="adjacent-input">
        <label for="minimize-launcher">
          <span class="label__title">{{ t('Settings.MinimizeLauncher') }}</span>
          <span class="label__description">{{ t('Settings.MinimizeTheLauncher') }}</span>
        </label>
        <Toggle
          id="minimize-launcher"
          :model-value="settings.hide_on_process"
          :checked="settings.hide_on_process"
          @update:model-value="
            (event) => {
              settings.hide_on_process = event
            }
          "
        />
      </div>

      <div v-if="!isMacOS" class="adjacent-input">
        <label for="native-decorations">
          <span class="label__title">{{ t('Settings.NativeDecorations') }}</span>
          <span class="label__description">{{ t('Settings.UseSystemWindowFrame') }}</span>
        </label>
        <Toggle
          id="native-decorations"
          :model-value="settings.native_decorations"
          :checked="settings.native_decorations"
          @update:model-value="
            (event) => {
              settings.native_decorations = event
            }
          "
        />
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

.language-dropdown {
  text-transform: capitalize;
}
</style>
