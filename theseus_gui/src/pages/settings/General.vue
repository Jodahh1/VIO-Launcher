<script setup>
import { Button, Card, FolderSearchIcon, BoxIcon, UpdatedIcon } from 'omorphia'
import { i18n } from '@/main.js'
import { useSettingsContext } from './context'

const t = i18n.global.t

const { settingsDir, findLauncherDir, refreshDir } = useSettingsContext()
</script>

<template>
  <div class="settings-panel">
    <Card class="settings-card">
      <div class="label">
        <h3>
          <span class="label__title size-card-header">{{ t('Settings.GeneralSettings') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.GeneralSectionDescription') }}</span>
      </div>

      <div class="adjacent-input">
        <label for="appDir">
          <span class="label__title">{{ t('Settings.AppDirectory') }}</span>
          <span class="label__description">
            {{ t('Settings.TheDirectoryWhereTheLauncherStoresAllOfItsFiles') }}
          </span>
        </label>
        <div class="app-directory">
          <div class="iconified-input">
            <BoxIcon />
            <input id="appDir" v-model="settingsDir" type="text" class="input" />
            <Button class="r-btn" @click="findLauncherDir">
              <FolderSearchIcon />
            </Button>
          </div>
          <Button large @click="refreshDir">
            <UpdatedIcon />
            {{ t('Settings.Refresh') }}
          </Button>
        </div>
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

.app-directory {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--gap-sm);

  .iconified-input {
    flex-grow: 1;

    input {
      flex-basis: auto;
    }
  }
}

@media (max-width: 860px) {
  .app-directory {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
