<script setup>
import { Button, Card, DownloadIcon, Modal, UpdatedIcon } from 'omorphia'
import { version, patch_version } from '../../../package.json'
import { blockDownload, buildInstalling, forceRefreshRemote, hrefVio } from '@/helpers/update.js'
import { formatVersionLabel } from '@/helpers/branding.js'
import { i18n } from '@/main.js'
import { ref } from 'vue'

const t = i18n.global.t

const confirmUpdate = ref(null)

await forceRefreshRemote(false, false)

const confirmUpdating = async () => {
  confirmUpdate.value.show()
}

const approvedUpdating = async () => {
  confirmUpdate.value.hide()
  await forceRefreshRemote(true, true)
}
</script>

<template>
  <div class="settings-panel">
    <Card class="settings-card">
      <div class="label inline-label">
        <h3>
          <span class="label__title size-card-header">{{ t('Settings.About') }}</span>
        </h3>
        <span class="label__description">{{ t('Settings.AboutSectionDescription') }}</span>
      </div>

      <label class="about-copy">
        <span class="label__title">VIO Launcher Version</span>
        <span class="label__description">
          Launcher version: {{ formatVersionLabel(version, patch_version) }}
        </span>
        <span class="label__description">
          {{ t('Settings.LatestAvailable') }}
          <a class="github-link" :href="hrefVio" target="_blank" rel="noopener noreferrer">
            {{ t('Settings.OurGithub') }}
          </a>
        </span>
        <span class="label__title">{{ t('Settings.UpdateChecker') }}</span>
        <span class="label__description">
          {{ t('Settings.Remote') }}
          <span id="releaseData" class="version-accent"></span>
        </span>
        <span class="label__description">
          {{ t('Settings.Local') }}
          <span class="version-accent">{{ formatVersionLabel(version, patch_version) }}</span>
        </span>
      </label>

      <div class="actions">
        <Button :disabled="blockDownload || buildInstalling" class="download-button" @click="confirmUpdating()">
          <DownloadIcon />
          {{ buildInstalling ? t('RunningAppBar.UpdateDownloading') : t('Settings.DownloadButton') }}
        </Button>
        <Button icon-only @click="forceRefreshRemote(false, false)">
          <UpdatedIcon />
        </Button>
      </div>

      <Modal ref="confirmUpdate" :has-to-type="false" :header="t('RunningAppBar.UpdatingHeader')">
        <div class="modal-body">
          <div class="markdown-body">
            <p>{{ t('RunningAppBar.UpdatingDesc') }}</p>
          </div>
          <div class="button-group push-right">
            <Button @click="confirmUpdate.hide()">{{ t('RunningAppBar.RejectUpdating') }}</Button>
            <Button @click="approvedUpdating()">{{ t('RunningAppBar.AcceptUpdating') }}</Button>
          </div>
        </div>
      </Modal>
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

.inline-label {
  margin-bottom: 0.25rem;
}

.about-copy {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.actions {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.download-button {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}

.version-accent,
.github-link {
  color: var(--color-brand);
  font-weight: 700;
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: var(--gap-lg);
  text-align: left;
}

.button-group {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}
</style>
