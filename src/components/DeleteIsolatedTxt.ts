import {ref} from 'vue'

import {invoke} from '@tauri-apps/api/tauri'

export const showDialog = ref(false)
export const deleteList = ref<string[]>([])
export let deleteDir: string = ''

export async function deleteIsolatedTxt(path: string) {
  deleteDir = path
  deleteList.value = await invoke('list_isolated_txt', {path: path})
  showDialog.value = true
}
