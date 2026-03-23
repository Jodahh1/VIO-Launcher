<script setup>
import { Card, Slider } from 'omorphia'
import JavaSelector from '@/components/ui/JavaSelector.vue'
import { i18n } from '@/main.js'
import { useSettingsContext } from './context'

const t = i18n.global.t

const { settings, maxMemory } = useSettingsContext()
</script>

<template>
  <div class="settings-panel">
    <Card class="settings-card">
      <div class="label">
        <h3>
          <span class="label__title size-card-header">{{ t('Settings.JavaSet') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.JavaSectionDescription') }}</span>
      </div>

      <label for="java-21">
        <span class="label__title">{{ t('Settings.Java21Location') }}</span>
      </label>
      <JavaSelector id="java-21" v-model="settings.java_globals.JAVA_21" :version="21" />

      <label for="java-17">
        <span class="label__title">{{ t('Settings.Java17Location') }}</span>
      </label>
      <JavaSelector id="java-17" v-model="settings.java_globals.JAVA_17" :version="17" />

      <label for="java-8">
        <span class="label__title">{{ t('Settings.Java8Location') }}</span>
      </label>
      <JavaSelector id="java-8" v-model="settings.java_globals.JAVA_8" :version="8" />

      <hr class="card-divider" />

      <label for="java-args">
        <span class="label__title">{{ t('Settings.JavaArgs') }}</span>
      </label>
      <input
        id="java-args"
        v-model="settings.javaArgs"
        autocomplete="off"
        type="text"
        class="installation-input"
        :placeholder="t('Settings.EnterJavaArgs')"
      />

      <label for="env-vars">
        <span class="label__title">{{ t('Settings.EnvVars') }}</span>
      </label>
      <input
        id="env-vars"
        v-model="settings.envArgs"
        autocomplete="off"
        type="text"
        class="installation-input"
        :placeholder="t('Settings.EnterEnvVars')"
      />

      <hr class="card-divider" />

      <div class="adjacent-input">
        <label for="max-memory">
          <span class="label__title">{{ t('Settings.JavaMem') }}</span>
          <span class="label__description">{{ t('Settings.JavaMemDesc') }}</span>
        </label>
        <Slider
          id="max-memory"
          v-model="settings.memory.maximum"
          :min="512"
          :max="maxMemory"
          :step="64"
          unit="mb"
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

.installation-input {
  width: 100% !important;
}
</style>
