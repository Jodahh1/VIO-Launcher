import { createRouter, createWebHistory } from 'vue-router'
import * as Pages from '@/pages'
import * as Project from '@/pages/project'
import * as Instance from '@/pages/instance'
import * as SettingsPages from '@/pages/settings'

/**
 * Configures application routing. Add page to pages/index and then add to route table here.
 */
export default new createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: Pages.Index,
      meta: {
        breadcrumb: [{ name: 'Home' }],
      },
    },
    {
      path: '/browse/:projectType',
      name: 'Browse',
      component: Pages.Browse,
      meta: {
        breadcrumb: [{ name: 'Browse' }],
      },
    },
    {
      path: '/library',
      name: 'Library',
      component: Pages.Library,
      meta: {
        breadcrumb: [{ name: 'Library' }],
      },
    },
    {
      path: '/settings',
      name: 'Settings',
      component: Pages.Settings,
      meta: {
        breadcrumb: [{ name: 'Settings' }],
      },
      children: [
        {
          path: '',
          redirect: '/settings/general',
        },
        {
          path: 'general',
          name: 'SettingsGeneral',
          component: SettingsPages.General,
          meta: {
            breadcrumb: [{ name: 'Settings', link: '/settings/general' }, { name: 'General' }],
          },
        },
        {
          path: 'themes',
          name: 'SettingsThemes',
          component: SettingsPages.Themes,
          meta: {
            breadcrumb: [
              { name: 'Settings', link: '/settings/general' },
              { name: 'Themes and customization' },
            ],
          },
        },
        {
          path: 'behavior',
          name: 'SettingsBehavior',
          component: SettingsPages.Behavior,
          meta: {
            breadcrumb: [
              { name: 'Settings', link: '/settings/general' },
              { name: 'Launcher behavior' },
            ],
          },
        },
        {
          path: 'performance',
          name: 'SettingsPerformance',
          component: SettingsPages.Performance,
          meta: {
            breadcrumb: [
              { name: 'Settings', link: '/settings/general' },
              { name: 'Performance' },
            ],
          },
        },
        {
          path: 'java',
          name: 'SettingsJava',
          component: SettingsPages.Java,
          meta: {
            breadcrumb: [{ name: 'Settings', link: '/settings/general' }, { name: 'Java' }],
          },
        },
        {
          path: 'window',
          name: 'SettingsWindow',
          component: SettingsPages.Window,
          meta: {
            breadcrumb: [{ name: 'Settings', link: '/settings/general' }, { name: 'Window' }],
          },
        },
        {
          path: 'hooks',
          name: 'SettingsHooks',
          component: SettingsPages.Hooks,
          meta: {
            breadcrumb: [{ name: 'Settings', link: '/settings/general' }, { name: 'Hooks' }],
          },
        },
        {
          path: 'privacy',
          name: 'SettingsPrivacy',
          component: SettingsPages.Privacy,
          meta: {
            breadcrumb: [{ name: 'Settings', link: '/settings/general' }, { name: 'Privacy' }],
          },
        },
        {
          path: 'about',
          name: 'SettingsAbout',
          component: SettingsPages.About,
          meta: {
            breadcrumb: [{ name: 'Settings', link: '/settings/general' }, { name: 'About' }],
          },
        },
      ],
    },
    {
      path: '/project/:id',
      name: 'Project',
      component: Project.Index,
      props: true,
      children: [
        {
          path: '',
          name: 'Description',
          component: Project.Description,
          meta: {
            useContext: true,
            breadcrumb: [{ name: '?Project' }],
          },
        },
        {
          path: 'versions',
          name: 'Versions',
          component: Project.Versions,
          meta: {
            useContext: true,
            breadcrumb: [{ name: '?Project', link: '/project/{id}/' }, { name: 'Versions' }],
          },
        },
        {
          path: 'version/:version',
          name: 'Version',
          component: Project.Version,
          props: true,
          meta: {
            useContext: true,
            breadcrumb: [
              { name: '?Project', link: '/project/{id}/' },
              { name: 'Versions', link: '/project/{id}/versions' },
              { name: '?Version' },
            ],
          },
        },
        {
          path: 'gallery',
          name: 'Gallery',
          component: Project.Gallery,
          meta: {
            useContext: true,
            breadcrumb: [{ name: '?Project', link: '/project/{id}/' }, { name: 'Gallery' }],
          },
        },
      ],
    },
    {
      path: '/instance/:id',
      name: 'Instance',
      component: Instance.Index,
      props: true,
      children: [
        {
          path: '',
          name: 'Mods',
          component: Instance.Mods,
          meta: {
            useRootContext: true,
            breadcrumb: [{ name: '?Instance' }],
          },
        },
        {
          path: 'projects/:type',
          name: 'ModsFilter',
          component: Instance.Mods,
          meta: {
            useRootContext: true,
            breadcrumb: [{ name: '?Instance' }],
          },
        },
        {
          path: 'options',
          name: 'Options',
          component: Instance.Options,
          meta: {
            useRootContext: true,
            breadcrumb: [{ name: '?Instance', link: '/instance/{id}/' }, { name: 'Options' }],
          },
        },
        {
          path: 'logs',
          name: 'Logs',
          component: Instance.Logs,
          meta: {
            useRootContext: true,
            breadcrumb: [{ name: '?Instance', link: '/instance/{id}/' }, { name: 'Logs' }],
          },
        },
      ],
    },
  ],
  linkActiveClass: 'router-link-active',
  linkExactActiveClass: 'router-link-exact-active',
  scrollBehavior() {
    // always scroll to top
    return { top: 0 }
  },
})
