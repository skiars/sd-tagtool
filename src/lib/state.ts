import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/tauri'

export const translate = ref(false)

export const tagPalette = ref<Map<string, string>>(new Map)

const defaultConfig = {
  translate: {
    language: 'zh-CN'
  }
}

export const config = ref(defaultConfig)

invoke('load_config').then(e => {
  console.log('load settings', e)
  config.value = {...defaultConfig, ...e as (typeof defaultConfig)}
})
