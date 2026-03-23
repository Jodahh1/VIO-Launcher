<script setup>
import { provide, ref, watch } from 'vue'
import { RouterLink, RouterView, useRoute } from 'vue-router'
import { Card } from 'omorphia'
import { handleError, useTheming } from '@/store/state'
import { change_config_dir, get, is_dir_writeable, set } from '@/helpers/settings'
import { get_max_memory } from '@/helpers/jre'
import { open } from '@tauri-apps/api/dialog'
import { getOS } from '@/helpers/utils.js'
import { useLanguage } from '@/store/language.js'
import { i18n } from '@/main.js'
import { settingsContextKey } from '@/pages/settings/context'
import { mixpanel_opt_in_tracking, mixpanel_opt_out_tracking } from '@/helpers/mixpanel'

const t = i18n.global.t

const pageOptions = ['Home', 'Library']

const themeStore = useTheming()
const languageStore = useLanguage()

const accessSettings = async () => {
  const fetchedSettings = await get()

  fetchedSettings.javaArgs = fetchedSettings.custom_java_args.join(' ')
  fetchedSettings.envArgs = fetchedSettings.custom_env_args.map((item) => item.join('=')).join(' ')

  return fetchedSettings
}

const initialSettings = await accessSettings().catch(handleError)
const settings = ref(initialSettings)
const settingsDir = ref(settings.value.loaded_config_dir)
const maxMemory = ref(Math.floor((await get_max_memory().catch(handleError)) / 1024))
const os = await getOS().catch(() => 'Windows')

watch(
  settings,
  async (newSettings, previousSettings) => {
    if (previousSettings && previousSettings.loaded_config_dir !== newSettings.loaded_config_dir) {
      return
    }

    const setSettings = JSON.parse(JSON.stringify(newSettings))

    if (setSettings.opt_out_analytics) {
      mixpanel_opt_out_tracking()
    } else {
      mixpanel_opt_in_tracking()
    }

    for (const value of Object.values(setSettings.java_globals)) {
      if (value?.path === '') {
        value.path = undefined
      }

      if (value?.path) {
        value.path = value.path.replace('java.exe', 'javaw.exe')
      }
    }

    setSettings.custom_java_args = setSettings.javaArgs.trim().split(/\s+/).filter(Boolean)
    setSettings.custom_env_args = setSettings.envArgs
      .trim()
      .split(/\s+/)
      .filter(Boolean)
      .map((value) => value.split('=').filter(Boolean))

    if (!setSettings.hooks.pre_launch) {
      setSettings.hooks.pre_launch = null
    }
    if (!setSettings.hooks.wrapper) {
      setSettings.hooks.wrapper = null
    }
    if (!setSettings.hooks.post_exit) {
      setSettings.hooks.post_exit = null
    }

    await set(setSettings)
  },
  { deep: true },
)

async function findLauncherDir() {
  const newDir = await open({
    multiple: false,
    directory: true,
    title: t('Settings.SelectANewAppDirectory'),
  })

  if (!newDir || Array.isArray(newDir)) {
    return
  }

  const writeable = await is_dir_writeable(newDir)

  if (!writeable) {
    handleError('The selected directory does not have proper permissions for write access.')
    return
  }

  settingsDir.value = newDir
  await refreshDir()
}

async function refreshDir() {
  await change_config_dir(settingsDir.value)
  settings.value = await accessSettings().catch(handleError)
  settingsDir.value = settings.value.loaded_config_dir
  themeStore.setThemeState(settings.value.theme)
  languageStore.setLanguageState(settings.value.language)
  themeStore.setCollapsedNavigation(settings.value.collapsed_navigation)
  themeStore.advancedRendering = settings.value.advanced_rendering
}

const sections = [
  {
    to: '/settings/general',
    titleKey: 'Settings.NavGeneral',
    descriptionKey: 'Settings.NavGeneralDesc',
  },
  {
    to: '/settings/themes',
    titleKey: 'Settings.NavThemesCustomization',
    descriptionKey: 'Settings.NavThemesCustomizationDesc',
  },
  {
    to: '/settings/behavior',
    titleKey: 'Settings.NavBehavior',
    descriptionKey: 'Settings.NavBehaviorDesc',
  },
  {
    to: '/settings/performance',
    titleKey: 'Settings.NavPerformance',
    descriptionKey: 'Settings.NavPerformanceDesc',
  },
  {
    to: '/settings/java',
    titleKey: 'Settings.NavJava',
    descriptionKey: 'Settings.NavJavaDesc',
  },
  {
    to: '/settings/window',
    titleKey: 'Settings.NavWindow',
    descriptionKey: 'Settings.NavWindowDesc',
  },
  {
    to: '/settings/hooks',
    titleKey: 'Settings.NavHooks',
    descriptionKey: 'Settings.NavHooksDesc',
  },
  {
    to: '/settings/privacy',
    titleKey: 'Settings.NavPrivacy',
    descriptionKey: 'Settings.NavPrivacyDesc',
  },
  {
    to: '/settings/about',
    titleKey: 'Settings.NavAbout',
    descriptionKey: 'Settings.NavAboutDesc',
  },
]

provide(settingsContextKey, {
  settings,
  settingsDir,
  maxMemory,
  pageOptions,
  themeStore,
  languageStore,
  isMacOS: os === 'MacOS',
  findLauncherDir,
  refreshDir,
})

const route = useRoute()
</script>

<template>
  <div class="settings-layout">
    <aside class="settings-sidebar">
      <Card class="settings-sidebar-card">
        <div class="label">
          <h3>
            <span class="label__title size-card-header">{{ t('Application.Settings') }}</span>
          </h3>
          <span class="label__description">
            {{ t('Settings.SettingsHubDescription') }}
          </span>
        </div>
        <div class="settings-nav">
          <RouterLink
            v-for="section in sections"
            :key="section.to"
            :to="section.to"
            class="settings-nav__item"
            :class="{ 'settings-nav__item--active': route.path === section.to }"
          >
            <span class="settings-nav__title">{{ t(section.titleKey) }}</span>
            <span class="settings-nav__description">{{ t(section.descriptionKey) }}</span>
          </RouterLink>
        </div>
      </Card>
    </aside>
    <section class="settings-content">
      <RouterView v-slot="{ Component }">
        <template v-if="Component">
          <Suspense>
            <component :is="Component"></component>
          </Suspense>
        </template>
      </RouterView>
    </section>
  </div>
</template>

<style scoped lang="scss">
.settings-layout {
  display: grid;
  grid-template-columns: minmax(17rem, 20rem) minmax(0, 1fr);
  gap: 1rem;
  padding: 1rem;
  min-height: 100%;
}

.settings-sidebar {
  position: sticky;
  top: 1rem;
  align-self: start;
}

.settings-sidebar-card {
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--color-raised-bg) 86%, white 14%) 0%, var(--color-raised-bg) 100%);
  border: 1px solid var(--color-button-bg);
}

.settings-nav {
  display: flex;
  flex-direction: column;
  gap: 0.65rem;
}

.settings-nav__item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.9rem 1rem;
  border-radius: var(--radius-lg);
  color: var(--color-base);
  background: color-mix(in srgb, var(--color-button-bg) 70%, transparent 30%);
  border: 1px solid transparent;
  transition: 0.15s ease-in-out;
}

.settings-nav__item:hover,
.settings-nav__item:focus-visible {
  color: var(--color-contrast);
  background: var(--color-button-bg-hover);
  border-color: color-mix(in srgb, var(--color-brand) 28%, transparent 72%);
}

.settings-nav__item--active,
.settings-nav__item.router-link-active {
  color: var(--color-contrast);
  background: linear-gradient(135deg, var(--color-brand-highlight) 0%, var(--color-button-bg) 100%);
  border-color: color-mix(in srgb, var(--color-brand) 40%, transparent 60%);
  box-shadow: 0 0 0 1px color-mix(in srgb, var(--color-brand) 22%, transparent 78%);
}

.settings-nav__title {
  font-weight: 700;
}

.settings-nav__description {
  font-size: 0.92rem;
  opacity: 0.76;
  line-height: 1.35;
}

.settings-content {
  min-width: 0;
}

@media (max-width: 1080px) {
  .settings-layout {
    grid-template-columns: 1fr;
  }

  .settings-sidebar {
    position: static;
  }

  .settings-nav {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(13rem, 1fr));
  }
}
</style>
