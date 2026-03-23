<script setup>
import { Card } from 'omorphia'
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
          <span class="label__title size-card-header">{{ t('Settings.Hooks') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.HooksSectionDescription') }}</span>
      </div>

      <div class="adjacent-input">
        <label for="pre-launch">
          <span class="label__title">{{ t('Settings.PreLaunch') }}</span>
          <span class="label__description">{{ t('Settings.PreLaunchDesc') }}</span>
        </label>
        <input
          id="pre-launch"
          v-model="settings.hooks.pre_launch"
          autocomplete="off"
          type="text"
          :placeholder="t('Settings.EnterPreLaunch')"
        />
      </div>

      <div class="adjacent-input">
        <label for="wrapper">
          <span class="label__title">{{ t('Settings.Wrapper') }}</span>
          <span class="label__description">{{ t('Settings.WrapperDesc') }}</span>
        </label>
        <input
          id="wrapper"
          v-model="settings.hooks.wrapper"
          autocomplete="off"
          type="text"
          :placeholder="t('Settings.EnterWrapper')"
        />
      </div>

      <div class="adjacent-input">
        <label for="post-exit">
          <span class="label__title">{{ t('Settings.PostExit') }}</span>
          <span class="label__description">{{ t('Settings.PostExitDesc') }}</span>
        </label>
        <input
          id="post-exit"
          v-model="settings.hooks.post_exit"
          autocomplete="off"
          type="text"
          :placeholder="t('Settings.EnterPostExit')"
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
