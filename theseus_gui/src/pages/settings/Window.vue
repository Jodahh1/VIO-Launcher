<script setup>
import { Card, Toggle } from 'omorphia'
import { i18n } from '@/main.js'
import { useSettingsContext } from './context'

const t = i18n.global.t

const { settings } = useSettingsContext()
</script>

<template>
  <div class="settings-panel">
    <Card class="settings-card">
      <div class="label">
        <h3>
          <span class="label__title size-card-header">{{ t('Settings.WindowSize') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.WindowSectionDescription') }}</span>
      </div>

      <div class="adjacent-input">
        <label for="fullscreen">
          <span class="label__title">{{ t('Settings.FullScreen') }}</span>
          <span class="label__description">{{ t('Settings.FullScreenDesc') }}</span>
        </label>
        <Toggle
          id="fullscreen"
          :model-value="settings.force_fullscreen"
          :checked="settings.force_fullscreen"
          @update:model-value="
            (event) => {
              settings.force_fullscreen = event
            }
          "
        />
      </div>

      <div class="adjacent-input">
        <label for="width">
          <span class="label__title">{{ t('Settings.Width') }}</span>
          <span class="label__description">{{ t('Settings.WidthDesc') }}</span>
        </label>
        <input
          id="width"
          v-model="settings.game_resolution[0]"
          :disabled="settings.force_fullscreen"
          autocomplete="off"
          type="number"
          :placeholder="t('Settings.EnterWidth')"
        />
      </div>

      <div class="adjacent-input">
        <label for="height">
          <span class="label__title">{{ t('Settings.Height') }}</span>
          <span class="label__description">{{ t('Settings.HeightDesc') }}</span>
        </label>
        <input
          id="height"
          v-model="settings.game_resolution[1]"
          :disabled="settings.force_fullscreen"
          autocomplete="off"
          type="number"
          :placeholder="t('Settings.EnterHeight')"
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
</style>
