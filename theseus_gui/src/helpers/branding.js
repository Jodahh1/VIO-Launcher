import packageInfo from '../../package.json'

export const supportTelegram = packageInfo.support_telegram
export const releaseRepository = packageInfo.release_repository
export const releaseRepositoryUrl = packageInfo.release_repository_url
export const releaseApiUrl = packageInfo.release_api_url

export function formatVersionLabel(version, patchVersion) {
  const patch = `${patchVersion ?? ''}`.trim()

  if (!patch) {
    return `v${version}`
  }

  return `v${version} - ${patch}`
}
