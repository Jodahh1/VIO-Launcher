<template>
  <Button
    v-if="props.mode !== 'isolated'"
    ref="button"
    v-tooltip.right="t('AccountsCard.MinecraftAccounts')"
    class="btn btn-transparent collapsed-button"
    @click="showCard = !showCard"
  >
    <Avatar
      size="xs"
      class="collapsed-button__icon"
      :src="selectedAccount ? avatarUrl(selectedAccount) : defaultAvatar"
    />
    <span class="collapsed-button__label">{{ t('AccountsCard.MinecraftAccounts') }}</span>
  </Button>
  <transition name="fade">
    <Card
      v-if="showCard || props.mode === 'isolated'"
      ref="card"
      class="account-card"
      :class="{ expanded: props.mode === 'expanded', isolated: props.mode === 'isolated' }"
    >
      <div v-if="selectedAccount" class="selected account">
        <Avatar size="xs" :src="avatarUrl(selectedAccount)" />
        <div class="account-copy">
          <h4>{{ selectedAccount.username }}</h4>
          <div class="account-meta">
            <span class="provider-badge" :class="providerClass(selectedAccount)">
              {{ providerLabel(selectedAccount) }}
            </span>
            <p>{{ t('AccountsCard.Active') }}</p>
          </div>
        </div>
        <Button
          v-tooltip="t('AccountsCard.Logout')"
          class="trash-icon-selected-fix"
          icon-only
          color="raised"
          @click="logout(selectedAccount.id)"
        >
          <TrashIcon />
        </Button>
      </div>

      <div v-else-if="displayAccounts.length > 0" class="logged-out no-account">
        <h4>{{ t('AccountsCard.NoSelected') }}</h4>
      </div>

      <div v-else class="logged-out no-account">
        <div class="empty-copy">
          <h4>{{ t('AccountsCard.NoAccount') }}</h4>
          <p>Sign in with Microsoft, Ely.by, or create an offline profile.</p>
        </div>
        <div class="account-no-account-fix">
          <Button v-tooltip="'Log in with Microsoft'" icon-only color="secondary" @click="loginMicrosoft()">
            <Microsoft class="account-type-no-account" />
          </Button>
          <Button v-tooltip="'Log in with Ely.by'" color="secondary" @click="openElyLogin()">
            <span class="provider-mark">Ely</span>
          </Button>
          <Button v-tooltip="'Add offline account'" color="secondary" @click="openOfflineLogin()">
            <span class="provider-mark">Off</span>
          </Button>
        </div>
      </div>

      <div v-if="displayAccounts.length > 0" class="account-group">
        <div v-for="account in displayAccounts" :key="account.id" class="account-row">
          <Button class="option account" @click="setAccount(account)">
            <Avatar :src="avatarUrl(account)" class="icon" />
            <div class="account-copy">
              <p class="account-name">{{ account.username }}</p>
              <span class="provider-badge" :class="providerClass(account)">
                {{ providerLabel(account) }}
              </span>
            </div>
          </Button>
          <Button
            v-tooltip="t('AccountsCard.Logout')"
            class="account-buttons-fix"
            icon-only
            @click="logout(account.id)"
          >
            <TrashIcon />
          </Button>
        </div>
      </div>

      <div v-if="accounts.length > 0" class="logged-out account-actions account">
        <Button @click="loginMicrosoft()">
          <Microsoft />
          Microsoft
        </Button>
        <Button @click="openElyLogin()">
          <span class="provider-mark">Ely</span>
          Ely.by
        </Button>
        <Button @click="openOfflineLogin()">
          <span class="provider-mark">Off</span>
          Offline
        </Button>
      </div>
    </Card>
  </transition>

  <Modal ref="loginOfflineModal" class="modal" header="Offline account">
    <div class="modal-body">
      <div class="modal-fields">
        <div class="label">Minecraft username</div>
        <input v-model="playerName" type="text" placeholder="Enter username" />
      </div>
      <Button color="secondary" @click="tryLoginOffline()">
        <PlusIcon />
        Add
      </Button>
    </div>
  </Modal>

  <Modal ref="elyLoginModal" class="modal" header="Ely.by account">
    <div class="modal-body modal-body--stack">
      <div class="modal-fields">
        <div class="label">Username or email</div>
        <input v-model="elyUsername" type="text" placeholder="Enter Ely.by username or email" />
      </div>
      <div class="modal-fields">
        <div class="label">Password</div>
        <input v-model="elyPassword" type="password" placeholder="Enter Ely.by password" />
      </div>
      <div class="button-row push-right">
        <Button @click="elyLoginModal.hide()">Close</Button>
        <Button color="primary" :disabled="elyLoading" @click="tryLoginEly()">
          {{ elyLoading ? 'Signing in...' : 'Sign in with Ely.by' }}
        </Button>
      </div>
    </div>
  </Modal>

  <Modal ref="elyTwoFactorModal" class="modal" header="Ely.by two-factor code">
    <div class="modal-body modal-body--stack">
      <div class="modal-fields">
        <div class="label">Authenticator code</div>
        <input v-model="elyTotpCode" type="text" maxlength="8" placeholder="Enter the 6-digit code" />
      </div>
      <div v-if="elyTotpError" class="label label-error">{{ elyTotpError }}</div>
      <div class="button-row push-right">
        <Button @click="elyTwoFactorModal.hide()">Close</Button>
        <Button color="primary" :disabled="elyLoading" @click="tryLoginElyWithTwoFactor()">
          {{ elyLoading ? 'Checking...' : 'Continue' }}
        </Button>
      </div>
    </div>
  </Modal>

  <Modal ref="loginErrorModal" class="modal" header="Input error">
    <div class="modal-body modal-body--stack">
      <div class="label">
        Unfortunately, we cannot accept such a username, please try again.
      </div>
      <Button color="primary" @click="tryAgainLoginOffline()">
        Try again
      </Button>
    </div>
  </Modal>

  <Modal ref="unexpectedErrorModal" class="modal" header="Error">
    <div class="modal-body modal-body--stack">
      <div class="label">{{ unexpectedErrorMessage }}</div>
    </div>
  </Modal>
</template>

<script setup>
import { i18n } from '@/main.js'
import { Avatar, Button, Card, Modal, PlusIcon, TrashIcon } from 'omorphia'
import { computed, onBeforeUnmount, onMounted, onUnmounted, ref } from 'vue'
import {
  offline_authenticate_await_completion,
  login as login_flow,
  remove_user,
  get_default_user,
  set_default_user,
  users,
} from '@/helpers/auth'
import { handleError } from '@/store/state.js'
import { mixpanel_track } from '@/helpers/mixpanel'
import { process_listener } from '@/helpers/events'
import { Microsoft } from '@/assets/render/index.js'
import { handleSevereError } from '@/store/error.js'
import {
  elyLogin,
  getElyAuthErrorMessage,
  isElyInvalidCredentialsError,
  isElyTwoFactorRequiredError,
} from '@/helpers/ely_auth.js'
import {
  getAccountAvatarUrl,
  getAccountProviderClass,
  getAccountProviderLabel,
  getDefaultAvatar,
} from '@/helpers/account.js'

const t = i18n.global.t

const props = defineProps({
  mode: {
    type: String,
    required: true,
    default: 'normal',
  },
})

const emit = defineEmits(['change'])

const accounts = ref([])
const loginOfflineModal = ref(null)
const elyLoginModal = ref(null)
const elyTwoFactorModal = ref(null)
const loginErrorModal = ref(null)
const unexpectedErrorModal = ref(null)
const playerName = ref('')
const defaultUser = ref(null)
const elyUsername = ref('')
const elyPassword = ref('')
const pendingElyUsername = ref('')
const pendingElyPassword = ref('')
const elyTotpCode = ref('')
const elyTotpError = ref('')
const elyLoading = ref(false)
const unexpectedErrorMessage = ref('An unexpected error occurred, please try again.')

const defaultAvatar = getDefaultAvatar()

async function refreshValues() {
  defaultUser.value = await get_default_user().catch(handleError)
  accounts.value = (await users().catch(handleError)) ?? []
}

defineExpose({
  refreshValues,
})

await refreshValues()

const displayAccounts = computed(() => accounts.value.filter((account) => defaultUser.value !== account.id))

const selectedAccount = computed(() => accounts.value.find((account) => account.id === defaultUser.value))

const avatarUrl = (account) => getAccountAvatarUrl(account)
const providerLabel = (account) => getAccountProviderLabel(account)
const providerClass = (account) => getAccountProviderClass(account)

function notifyAccountsChanged() {
  emit('change')
  window.dispatchEvent(new CustomEvent('vio:accounts-changed'))
}

async function setAccount(account) {
  defaultUser.value = account.id
  await set_default_user(account.id).catch(handleError)
  notifyAccountsChanged()
}

async function loginMicrosoft() {
  const loggedIn = await login_flow().catch(handleSevereError)

  if (loggedIn) {
    await setAccount(loggedIn)
    await refreshValues()
  }

  mixpanel_track('AccountLogIn')
}

function openOfflineLogin() {
  loginOfflineModal.value.show()
}

function openElyLogin() {
  elyTotpError.value = ''
  elyLoginModal.value.show()
}

async function tryLoginOffline() {
  const name = playerName.value.trim()

  if (name.length > 1 && name.length < 20) {
    const loggedIn = await offline_authenticate_await_completion(name).catch(handleError)
    loginOfflineModal.value.hide()

    if (loggedIn) {
      await setAccount(loggedIn)
      await refreshValues()
    } else {
      unexpectedErrorMessage.value = 'Unable to create the offline account right now.'
      unexpectedErrorModal.value.show()
    }

    playerName.value = ''
    mixpanel_track('AccountLogIn')
  } else {
    playerName.value = ''
    loginOfflineModal.value.hide()
    loginErrorModal.value.show()
  }
}

async function tryLoginEly() {
  const username = elyUsername.value.trim()
  const password = elyPassword.value

  if (!username || !password) {
    unexpectedErrorMessage.value = 'Enter your Ely.by username or email and password.'
    unexpectedErrorModal.value.show()
    return
  }

  try {
    elyLoading.value = true
    const loggedIn = await elyLogin(username, password)

    if (loggedIn) {
      elyLoginModal.value.hide()
      elyTwoFactorModal.value.hide()
      pendingElyUsername.value = ''
      pendingElyPassword.value = ''
      elyTotpCode.value = ''
      elyTotpError.value = ''
      await setAccount(loggedIn)
      await refreshValues()
      mixpanel_track('AccountLogIn')
    }
  } catch (error) {
    if (isElyTwoFactorRequiredError(error)) {
      pendingElyUsername.value = username
      pendingElyPassword.value = password
      elyTotpCode.value = ''
      elyTotpError.value = ''
      elyLoginModal.value.hide()
      elyTwoFactorModal.value.show()
      return
    }

    unexpectedErrorMessage.value = getElyAuthErrorMessage(error)
    unexpectedErrorModal.value.show()
  } finally {
    elyLoading.value = false
  }
}

async function tryLoginElyWithTwoFactor() {
  const username = pendingElyUsername.value.trim()
  const password = pendingElyPassword.value
  const totpCode = elyTotpCode.value.trim()

  if (!totpCode) {
    elyTotpError.value = 'Enter the code from your authenticator app.'
    return
  }

  try {
    elyLoading.value = true
    elyTotpError.value = ''
    const loggedIn = await elyLogin(username, password, totpCode)

    if (loggedIn) {
      elyTwoFactorModal.value.hide()
      elyLoginModal.value.hide()
      elyUsername.value = ''
      elyPassword.value = ''
      pendingElyUsername.value = ''
      pendingElyPassword.value = ''
      elyTotpCode.value = ''
      await setAccount(loggedIn)
      await refreshValues()
      mixpanel_track('AccountLogIn')
    }
  } catch (error) {
    if (isElyInvalidCredentialsError(error)) {
      elyTotpError.value = 'Invalid two-factor code. Please try again.'
      return
    }

    unexpectedErrorMessage.value = getElyAuthErrorMessage(error)
    unexpectedErrorModal.value.show()
  } finally {
    elyLoading.value = false
  }
}

function tryAgainLoginOffline() {
  loginErrorModal.value.hide()
  openOfflineLogin()
}

const logout = async (id) => {
  await remove_user(id).catch(handleError)
  await refreshValues()

  if (!selectedAccount.value && accounts.value.length > 0) {
    await setAccount(accounts.value[0])
    await refreshValues()
  } else {
    notifyAccountsChanged()
  }

  mixpanel_track('AccountLogOut')
}

const showCard = ref(false)
const card = ref(null)
const button = ref(null)

const handleClickOutside = (event) => {
  const elements = document.elementsFromPoint(event.clientX, event.clientY)
  const buttonEl = button.value?.$el ?? button.value

  if (
    card.value &&
    card.value.$el !== event.target &&
    !elements.includes(card.value.$el) &&
    buttonEl &&
    !buttonEl.contains(event.target)
  ) {
    showCard.value = false
  }
}

const unlisten = await process_listener(async (e) => {
  if (e.event === 'launched') {
    await refreshValues()
  }
})

onMounted(() => {
  window.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  window.removeEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  unlisten()
})
</script>

<style scoped lang="scss">
.selected {
  background: linear-gradient(135deg, var(--color-brand-highlight), rgba(255, 255, 255, 0.08));
  border-radius: var(--radius-lg);
  color: var(--color-contrast);
  gap: 1rem;
}

.logged-out {
  background: var(--color-bg);
  border-radius: var(--radius-lg);
  gap: 0.5rem;
}

.account {
  width: auto;
  display: flex;
  align-items: center;
  text-align: left;
  padding: 0.75rem;

  h4,
  p {
    margin: 0;
  }
}

.account-copy {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.account-meta {
  display: flex;
  align-items: center;
  gap: 0.5rem;

  p {
    color: rgba(255, 255, 255, 0.8);
  }
}

.account-name {
  margin: 0;
  font-weight: 600;
}

.provider-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: fit-content;
  padding: 0.15rem 0.55rem;
  border-radius: 999px;
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  border: 1px solid rgba(255, 255, 255, 0.14);
}

.provider-badge--microsoft {
  background: rgba(0, 120, 212, 0.18);
  color: #9fd0ff;
}

.provider-badge--ely {
  background: rgba(255, 121, 198, 0.18);
  color: #ffb8d6;
}

.provider-badge--offline {
  background: rgba(255, 255, 255, 0.08);
  color: var(--color-base);
}

.provider-mark {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 2rem;
  font-weight: 800;
  letter-spacing: 0.04em;
}

.account-type-no-account {
  display: inline-flex;
  margin-left: auto;
}

.account-actions {
  width: auto;
  margin: auto;
  gap: 0.5rem;
}

.account-no-account-fix {
  width: max-content;
  gap: 0.5rem;
  display: flex;
  margin-left: auto;
}

.account-buttons-fix {
  display: flex;
}

.trash-icon-selected-fix {
  display: flex;
  margin-left: auto;
}

.account-card {
  width: min-content;
  position: initial;
  display: flex;
  flex-direction: column;
  top: 0.5rem;
  left: auto;
  margin-top: inherit;
  z-index: 11;
  gap: 0.5rem;
  padding: 1rem;
  border: 1px solid var(--color-button-bg);
  user-select: none;
  -ms-user-select: none;
  -webkit-user-select: none;
  max-height: 40vh;
  overflow-y: auto;

  &::-webkit-scrollbar-track {
    border-top-right-radius: 1rem;
    border-bottom-right-radius: 1rem;
  }

  &::-webkit-scrollbar {
    border-top-right-radius: 1rem;
    border-bottom-right-radius: 1rem;
  }

  &.hidden {
    display: none;
  }

  &.expanded {
    left: 13.5rem;
  }

  &.isolated {
    position: relative;
    left: 0;
    top: 0;
  }
}

.account-group {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.option {
  background: var(--color-raised-bg);
  color: var(--color-base);
  box-shadow: none;
  width: 100%;
  justify-content: flex-start;

  img {
    margin-right: 0.5rem;
  }
}

.icon {
  --size: 1.5rem !important;
}

.account-row {
  display: flex;
  flex-direction: row;
  gap: 0.25rem;
  justify-content: space-between;
  padding-right: 0.5rem;
}

.empty-copy {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;

  p {
    color: var(--color-secondary-text, rgba(255, 255, 255, 0.72));
    font-size: 0.85rem;
  }
}

.no-account {
  width: 22rem;
  display: flex;
  align-items: center;
  text-align: left;
  padding: 0.75rem;

  h4,
  p {
    margin: 0;
  }
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.modal-body {
  display: flex;
  flex-direction: row;
  gap: var(--gap-lg);
  align-items: flex-end;
  padding: var(--gap-xl);
}

.modal-body--stack {
  flex-direction: column;
  align-items: stretch;
}

.modal-fields {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;
}

.label-error {
  color: #ff9ab8;
}

.button-row {
  display: flex;
  flex-direction: row;
  gap: 0.5rem;
  justify-content: flex-end;
}

.modal {
  position: absolute;
}
</style>
