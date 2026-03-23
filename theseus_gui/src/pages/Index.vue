<script setup>
import { Avatar, Button, Card } from 'omorphia'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import RowDisplay from '@/components/RowDisplay.vue'
import { offline_listener, profile_listener } from '@/helpers/events'
import { useBreadcrumbs } from '@/store/breadcrumbs'
import { useFetch } from '@/helpers/fetch.js'
import { isOffline } from '@/helpers/utils'
import { i18n } from '@/main.js'
import { forceRefreshRemote } from '@/helpers/update.js'
import { useInstances } from '@/store/instances'
import { storeToRefs } from 'pinia'
import { get_default_user, users } from '@/helpers/auth'
import {
  getAccountAvatarUrl,
  getAccountEmail,
  getAccountProviderLabel,
  getAccountRenderUrl,
} from '@/helpers/account.js'
import { openExternal } from '@/helpers/external.js'
import { supportTelegram } from '@/helpers/branding.js'

const t = i18n.global.t

const featuredModpacks = ref({})
const featuredMods = ref({})
const filter = ref('')
const accounts = ref([])
const defaultUser = ref(null)

const route = useRoute()
const router = useRouter()
const breadcrumbs = useBreadcrumbs()

breadcrumbs.setRootContext({ name: 'Home', link: route.path })

const offline = ref(await isOffline())

const instancesStore = useInstances()
const { instancesByPlayed } = storeToRefs(instancesStore)

const activeAccount = computed(() => {
  return (
    accounts.value.find((account) => account.id === defaultUser.value) ??
    accounts.value[0] ??
    null
  )
})

const activeAvatarUrl = computed(() => getAccountAvatarUrl(activeAccount.value, 256))
const activeRenderUrl = computed(() => getAccountRenderUrl(activeAccount.value, 300))
const activeProviderLabel = computed(() => getAccountProviderLabel(activeAccount.value))
const activeEmail = computed(() => getAccountEmail(activeAccount.value))
const heroTitle = computed(() =>
  activeAccount.value ? `Welcome back, ${activeAccount.value.username}` : 'Your Minecraft hub is ready',
)
const heroDescription = computed(() =>
  activeAccount.value
    ? `${activeProviderLabel.value} account connected. Jump back into your library, browse fresh content, or manage everything from one place.`
    : 'Sign in with Microsoft, Ely.by, or create an offline account to start building your VIO library.',
)

async function refreshAccounts() {
  defaultUser.value = await get_default_user().catch(() => null)
  accounts.value = (await users().catch(() => [])) ?? []
}

const getInstances = async () => {
  await instancesStore.refreshInstances()

  const filters = []
  for (const instance of instancesByPlayed.value) {
    if (instance.metadata.linked_data && instance.metadata.linked_data.project_id) {
      filters.push(`NOT"project_id"="${instance.metadata.linked_data.project_id}"`)
    }
  }

  filter.value = filters.join(' AND ')
}

const getFeaturedModpacks = async () => {
  const response = await useFetch(
    `https://api.modrinth.com/v2/search?facets=[["project_type:modpack"]]&limit=10&index=follows&filters=${filter.value}`,
    'featured modpacks',
    offline.value,
  )

  featuredModpacks.value = response ? response.hits : []
}

const getFeaturedMods = async () => {
  const response = await useFetch(
    'https://api.modrinth.com/v2/search?facets=[["project_type:mod"]]&limit=10&index=follows',
    'featured mods',
    offline.value,
  )

  featuredMods.value = response ? response.hits : []
}

await Promise.all([refreshAccounts(), getInstances()])
await Promise.all([getFeaturedModpacks(), getFeaturedMods()])

const unlistenProfile = await profile_listener(async (e) => {
  await Promise.all([refreshAccounts(), getInstances()])

  if (e.event === 'created' || e.event === 'removed') {
    await Promise.all([getFeaturedModpacks(), getFeaturedMods()])
  }
})

const unlistenOffline = await offline_listener(async (value) => {
  offline.value = value

  if (!value) {
    await Promise.all([getFeaturedModpacks(), getFeaturedMods()])
  }
})

const total = computed(() => {
  return (
    (instancesByPlayed.value?.length ?? 0) +
    (featuredModpacks.value?.length ?? 0) +
    (featuredMods.value?.length ?? 0)
  )
})

const handleRenderFallback = (event) => {
  event.target.src = activeAvatarUrl.value
}

const openSupport = () => openExternal(window, supportTelegram)
const refreshOnAccountChange = () => refreshAccounts()

onMounted(() => {
  window.addEventListener('vio:accounts-changed', refreshOnAccountChange)
})

onUnmounted(() => {
  window.removeEventListener('vio:accounts-changed', refreshOnAccountChange)
  unlistenProfile()
  unlistenOffline()
})

await forceRefreshRemote(true)
</script>

<template>
  <div class="page-container">
    <div class="hero-grid">
      <Card class="hero-card">
        <div class="hero-copy">
          <span class="hero-kicker">VIO launcher</span>
          <h1>{{ heroTitle }}</h1>
          <p class="hero-description">{{ heroDescription }}</p>

          <div class="hero-actions">
            <Button @click="router.push('/library')">Open library</Button>
            <Button @click="router.push('/browse/modpack')">Browse modpacks</Button>
            <Button @click="router.push('/settings')">Launcher settings</Button>
          </div>

          <div class="hero-stats">
            <div class="stat-pill">
              <span class="stat-pill__label">Accounts</span>
              <strong>{{ accounts.length }}</strong>
            </div>
            <div class="stat-pill">
              <span class="stat-pill__label">Instances</span>
              <strong>{{ instancesByPlayed.length }}</strong>
            </div>
            <div class="stat-pill">
              <span class="stat-pill__label">Recommended</span>
              <strong>{{ (featuredModpacks?.length ?? 0) + (featuredMods?.length ?? 0) }}</strong>
            </div>
          </div>

          <div v-if="activeAccount" class="account-summary">
            <span class="account-summary__label">Active account</span>
            <div class="account-summary__body">
              <Avatar :src="activeAvatarUrl" size="xs" />
              <div>
                <strong>{{ activeAccount.username }}</strong>
                <p>{{ activeProviderLabel }}</p>
                <p v-if="activeEmail">{{ activeEmail }}</p>
              </div>
            </div>
          </div>
        </div>

        <div class="hero-visual">
          <div class="skin-frame">
            <img
              v-if="activeAccount"
              :src="activeRenderUrl"
              :alt="`${activeAccount.username} skin preview`"
              class="skin-render"
              @error="handleRenderFallback"
            />
            <div v-else class="skin-placeholder">
              <Avatar :src="activeAvatarUrl" size="md" />
              <span>No Minecraft account selected yet</span>
            </div>
          </div>
        </div>
      </Card>

      <Card class="support-card">
        <span class="support-card__kicker">Support</span>
        <h3>@J0dah</h3>
        <p>
          Need help with sign-in, builds, skins, or launcher setup? Contact Telegram support
          directly.
        </p>
        <Button color="primary" @click="openSupport">Open Telegram</Button>
      </Card>
    </div>

    <RowDisplay
      v-if="total > 0"
      :instances="[
        {
          label: t('Instance.Index.JumpBackIn'),
          route: '/library',
          instances: instancesByPlayed,
          downloaded: true,
        },
        {
          label: t('Instance.Index.PopularPacks'),
          route: '/browse/modpack',
          instances: featuredModpacks,
          downloaded: false,
        },
        {
          label: t('Instance.Index.PopularMods'),
          route: '/browse/mod',
          instances: featuredMods,
          downloaded: false,
        },
      ]"
      :can-paginate="true"
    />
  </div>
</template>

<style lang="scss" scoped>
.page-container {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  width: 100%;
  gap: 1rem;
  padding: 1rem;
}

.hero-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.8fr) minmax(19rem, 0.9fr);
  gap: 1rem;
}

.hero-card {
  display: grid;
  grid-template-columns: minmax(0, 1.4fr) minmax(16rem, 0.8fr);
  gap: 1rem;
  padding: 1.25rem;
  overflow: hidden;
  background:
    radial-gradient(circle at top right, rgba(255, 126, 185, 0.2), transparent 32rem),
    linear-gradient(145deg, rgba(255, 255, 255, 0.04), rgba(255, 255, 255, 0.01));
}

.hero-copy {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  h1,
  p {
    margin: 0;
  }
}

.hero-kicker,
.support-card__kicker {
  display: inline-flex;
  width: fit-content;
  padding: 0.3rem 0.7rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.06);
  color: var(--color-brand);
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.hero-description {
  color: var(--color-secondary-text, rgba(255, 255, 255, 0.72));
  max-width: 42rem;
}

.hero-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.hero-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.stat-pill {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  min-width: 8rem;
  padding: 0.85rem 1rem;
  border-radius: var(--radius-lg);
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.05);
}

.stat-pill__label {
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--color-secondary-text, rgba(255, 255, 255, 0.58));
}

.account-summary {
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
  padding: 1rem;
  border-radius: var(--radius-lg);
  background: rgba(0, 0, 0, 0.16);
  border: 1px solid rgba(255, 255, 255, 0.06);
}

.account-summary__label {
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--color-secondary-text, rgba(255, 255, 255, 0.58));
}

.account-summary__body {
  display: flex;
  align-items: center;
  gap: 0.75rem;

  p,
  strong {
    margin: 0;
  }
}

.hero-visual {
  display: flex;
  align-items: stretch;
}

.skin-frame {
  width: 100%;
  min-height: 20rem;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: calc(var(--radius-xl) + 0.25rem);
  background:
    radial-gradient(circle at top, rgba(255, 135, 190, 0.22), transparent 60%),
    rgba(0, 0, 0, 0.22);
  border: 1px solid rgba(255, 255, 255, 0.06);
  overflow: hidden;
}

.skin-render {
  width: 100%;
  max-width: 18rem;
  height: auto;
  object-fit: contain;
  filter: drop-shadow(0 1rem 2rem rgba(255, 120, 185, 0.3));
}

.skin-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  color: var(--color-secondary-text, rgba(255, 255, 255, 0.72));
}

.support-card {
  display: flex;
  flex-direction: column;
  gap: 0.9rem;
  padding: 1.25rem;
  justify-content: space-between;
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.05), rgba(255, 255, 255, 0.015)),
    rgba(0, 0, 0, 0.16);

  h3,
  p {
    margin: 0;
  }

  p {
    color: var(--color-secondary-text, rgba(255, 255, 255, 0.72));
  }
}

@media (max-width: 1100px) {
  .hero-grid,
  .hero-card {
    grid-template-columns: 1fr;
  }
}
</style>
