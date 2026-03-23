const DEFAULT_AVATAR = 'https://launcher-files.modrinth.com/assets/steve_head.png'

export function getAccountProvider(account) {
  const rawType = account?.account_type?.type

  if (rawType === 'ElyBy') {
    return 'ely'
  }

  if (rawType === 'Offline' || account?.access_token === 'null') {
    return 'offline'
  }

  return 'microsoft'
}

export function getAccountProviderLabel(account) {
  const provider = getAccountProvider(account)

  if (provider === 'ely') {
    return 'Ely.by'
  }

  if (provider === 'offline') {
    return 'Offline'
  }

  return 'Microsoft'
}

export function getAccountProviderClass(account) {
  return `provider-badge--${getAccountProvider(account)}`
}

export function getAccountAvatarUrl(account, size = 128) {
  if (!account?.username) {
    return DEFAULT_AVATAR
  }

  if (getAccountProvider(account) === 'ely') {
    return `https://skinsystem.ely.by/avatars/${encodeURIComponent(account.username)}?size=${size}`
  }

  return `https://mc-heads.net/avatar/${encodeURIComponent(account.username)}/${size}`
}

export function getAccountRenderUrl(account, size = 256) {
  if (!account?.username) {
    return DEFAULT_AVATAR
  }

  if (getAccountProvider(account) === 'ely') {
    const profileId =
      account?.ely_data?.uuid ?? account?.account_type?.profile_id ?? account.username
    return `https://skinsystem.ely.by/front/${encodeURIComponent(profileId)}`
  }

  return `https://mc-heads.net/body/${encodeURIComponent(account.username)}/right/${size}`
}

export function getAccountEmail(account) {
  if (getAccountProvider(account) === 'ely') {
    return account?.ely_data?.email ?? null
  }

  return null
}

export function getDefaultAvatar() {
  return DEFAULT_AVATAR
}
