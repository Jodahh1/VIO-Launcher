import { ref } from 'vue'
import { patch_version, version } from '../../package.json'
import { downloadBuild, getOS } from '@/helpers/utils.js'
import { formatVersionLabel, releaseApiUrl, releaseRepositoryUrl } from '@/helpers/branding.js'

export const blockDownload = ref(true)
export const buildInstalling = ref(false)
export const updateAvailable = ref(false)
export const hrefVio = releaseRepositoryUrl

const os = ref('')
const apiUrl = releaseApiUrl
const failedPattern = 'Failed to fetch remote VIO releases:'
const v = 'v'
const localVersion = formatVersionLabel(version, patch_version)
const macExtension = '.dmg'
const windowsExtension = '.msi'
const linuxExtension = '.deb'
const blacklistDevBuilds = 'DEV_BUILD'

export async function forceRefreshRemote(disableElementId, autoUpdate) {
  fetch(apiUrl)
    .then((response) => {
      if (!response.ok) {
        throw new Error(
          response.status === 404
            ? 'GitHub releases are not configured for this repository yet.'
            : `Failed to fetch releases. Status: ${response.status}`,
        )
      }
      return response.json()
    })
    .then(async (data) => {
      os.value = await getOS()
      const latestRelease = data.tag_name || data.name
      let remoteVersion

      if (!disableElementId) {
        const releaseData = document.getElementById('releaseData')
        if (!releaseData) {
          console.error('Release data element not found.')
          return
        }
        releaseData.textContent = latestRelease
        remoteVersion = `${releaseData.textContent}`
      } else {
        remoteVersion = latestRelease
      }

      if (remoteVersion && remoteVersion.startsWith(localVersion)) {
        updateAvailable.value = false
        blockDownload.value = true
      } else if (remoteVersion && remoteVersion.startsWith(v)) {
        updateAvailable.value = true
        blockDownload.value = false
      } else {
        updateAvailable.value = false
        blockDownload.value = true
      }

      console.log('Update available state is', updateAvailable.value)
      console.log('Remote version is', remoteVersion)
      console.log('Operating System is', os.value)

      if (autoUpdate) {
        buildInstalling.value = true
        let downloadUrl
        let fileName
        const buildType = data.assets

        if (os.value.toLowerCase() === 'macos') {
          for (const item of buildType) {
            if (item.name.endsWith(macExtension) && !item.name.startsWith(blacklistDevBuilds)) {
              downloadUrl = item.browser_download_url
              fileName = item.name
              console.log(item.browser_download_url)
            }
          }

          await downloadBuild(downloadUrl, fileName, os.value, true)
        } else if (os.value.toLowerCase() === 'windows') {
          for (const item of buildType) {
            if (item.name.endsWith(windowsExtension) && !item.name.startsWith(blacklistDevBuilds)) {
              downloadUrl = item.browser_download_url
              fileName = item.name
              console.log(item.browser_download_url)
            }
          }

          await downloadBuild(downloadUrl, fileName, os.value, true)
        } else if (os.value.toLowerCase() === 'linux') {
          console.warn(
            "[VIO Warning] Due to some circumstances, we can't fully determine the structure and condition of your Linux OS," +
              " so we'll download the latest build for the latest ubuntu, that we've available. Installation is done manually",
          )
          for (const item of buildType) {
            if (item.name.endsWith(linuxExtension) && !item.name.startsWith(blacklistDevBuilds)) {
              downloadUrl = item.browser_download_url
              fileName = item.name
              console.log(item.browser_download_url)
            }
          }

          await downloadBuild(downloadUrl, fileName, os.value, false)
        }

        buildInstalling.value = false
        console.log(fileName)
        console.log(downloadUrl)
      }
    })
    .catch((error) => {
      console.error(failedPattern, error)
      if (!disableElementId) {
        const errorData = document.getElementById('releaseData')
        if (errorData) {
          errorData.textContent = `${error.message}`
        }
        updateAvailable.value = false
        blockDownload.value = true
      }
    })
}
