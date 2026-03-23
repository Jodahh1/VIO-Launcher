import { invoke } from '@tauri-apps/api/tauri'

function getSerializedErrorMessage(error) {
  if (typeof error === 'string') {
    return error
  }

  if (error?.message) {
    return error.message
  }

  return 'Unexpected Ely.by authentication error.'
}

function normalizeElyError(error) {
  const message = getSerializedErrorMessage(error)
  const normalized = new Error(message)

  if (message.startsWith('ELY_2FA_REQUIRED:')) {
    normalized.code = 'ELY_2FA_REQUIRED'
    normalized.userMessage = 'This Ely.by account is protected with two-factor authentication.'
  } else if (message.startsWith('ELY_INVALID_CREDENTIALS:')) {
    normalized.code = 'ELY_INVALID_CREDENTIALS'
    normalized.userMessage = 'Invalid Ely.by credentials.'
  } else if (message.startsWith('ELY_AUTH_ERROR:')) {
    normalized.code = 'ELY_AUTH_ERROR'
    normalized.userMessage = message.replace('ELY_AUTH_ERROR: ', '')
  } else {
    normalized.code = 'ELY_AUTH_UNKNOWN'
    normalized.userMessage = message
  }

  return normalized
}

export function isElyTwoFactorRequiredError(error) {
  return error?.code === 'ELY_2FA_REQUIRED'
}

export function isElyInvalidCredentialsError(error) {
  return error?.code === 'ELY_INVALID_CREDENTIALS'
}

export function getElyAuthErrorMessage(error) {
  return error?.userMessage ?? getSerializedErrorMessage(error)
}

export async function elyLogin(username, password, totpCode = '') {
  const combinedPassword = totpCode ? `${password}:${totpCode}` : password

  try {
    return await invoke('plugin:ely-auth|ely_login', {
      username,
      password: combinedPassword,
    })
  } catch (error) {
    throw normalizeElyError(error)
  }
}

export async function elyLogout(userId) {
  return await invoke('plugin:ely-auth|ely_logout', { userId })
}

export async function elyRefresh(userId) {
  return await invoke('plugin:ely-auth|ely_refresh', { userId })
}

export async function elyGetAccounts() {
  return await invoke('plugin:ely-auth|ely_get_accounts')
}

export async function elySetDefault(userId) {
  return await invoke('plugin:ely-auth|ely_set_default', { userId })
}

export async function elyRemoveAccount(userId) {
  return await invoke('plugin:ely-auth|ely_remove_account', { userId })
}

export async function elyEnsureAuthlib() {
  return await invoke('plugin:ely-auth|ely_ensure_authlib')
}

export async function elyCheckApiStatus() {
  return await invoke('plugin:ely-auth|ely_check_api_status')
}
