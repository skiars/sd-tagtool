<script setup lang="ts">
import {useRouter} from 'vue-router'
import Panel from 'primevue/panel'
import Dropdown from 'primevue/dropdown'
import {invoke} from '@tauri-apps/api/tauri'
import {config} from '../lib/state'

const router = useRouter()

const languages = [
  {name: 'English', code: 'en'},
  {name: 'Japanese', code: 'ja'},
  {name: 'Simplified Chinese', code: 'zh-CN'},
];

async function onBack() {
  await invoke('save_config', {model: config.value})
  await invoke('refresh_cache')
  router.back()
}
</script>

<template>
  <div>
    Settings
    <button v-on:click="onBack()">Back</button>
    <panel header="Translate" toggleable>
      Target language
      <dropdown v-model="config.translate.language" :options="languages" option-label="name" option-value="code"/>
    </panel>
  </div>
</template>

<style scoped>

</style>