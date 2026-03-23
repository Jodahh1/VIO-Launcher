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
          <span class="label__title size-card-header">{{ t('Settings.Privacy') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.PrivacySectionDescription') }}</span>
      </div>

      <div class="adjacent-input">
        <label for="opt-out-analytics">
          <span class="label__title">{{ t('Settings.DisableAnalytics') }}</span>
          <span class="label__description">{{ t('Settings.AnalyticsDesc') }}</span>
        </label>
        <Toggle
          id="opt-out-analytics"
          :model-value="settings.opt_out_analytics"
          :checked="settings.opt_out_analytics"
          @update:model-value="
            (event) => {
              settings.opt_out_analytics = event
            }
          "
        />
      </div>

      <div class="adjacent-input">
        <label for="disable-discord-rpc">
          <span class="label__title">{{ t('Settings.DisableRPC') }}</span>
          <span class="label__description">{{ t('Settings.DisableRPCDesc') }}</span>
        </label>
        <Toggle
          id="disable-discord-rpc"
          v-model="settings.disable_discord_rpc"
          :checked="settings.disable_discord_rpc"
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
