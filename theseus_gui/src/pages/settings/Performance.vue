<script setup>
import { Card, Slider } from 'omorphia'
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
          <span class="label__title size-card-header">{{ t('Settings.ResourceManagement') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.PerformanceSectionDescription') }}</span>
      </div>

      <div class="adjacent-input">
        <label for="max-downloads">
          <span class="label__title">{{ t('Settings.Mcd') }}</span>
          <span class="label__description">{{ t('Settings.McdDesc') }}</span>
        </label>
        <Slider
          id="max-downloads"
          v-model="settings.max_concurrent_downloads"
          :min="1"
          :max="10"
          :step="1"
        />
      </div>

      <div class="adjacent-input">
        <label for="max-writes">
          <span class="label__title">{{ t('Settings.Mcw') }}</span>
          <span class="label__description">{{ t('Settings.McwDesc') }}</span>
        </label>
        <Slider
          id="max-writes"
          v-model="settings.max_concurrent_writes"
          :min="1"
          :max="50"
          :step="1"
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
