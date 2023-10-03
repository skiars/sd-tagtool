<script setup lang="ts">
import {watch} from 'vue'
import {useRouter} from 'vue-router'
import Panel from 'primevue/panel'
import Dropdown from 'primevue/dropdown'
import Slider from 'primevue/slider'
import {invoke} from '@tauri-apps/api/tauri'
import {config} from '../lib/state'

const router = useRouter()

const languages = [
  {name: 'English', code: 'en'},
  {name: 'Japanese', code: 'ja'},
  {name: 'Simplified Chinese', code: 'zh-CN'},
];

watch(config, async value => {
    await invoke('save_config', {model: value})
    await invoke('refresh_cache')
  },
  {deep: true}
)
</script>

<template>
  <div class="settings-window">
    <div class="settings-header">
      <h2>
        <i class="pi pi-arrow-circle-left" v-on:click="router.back()"/>
        Settings
      </h2>
    </div>
    <div class="settings-panel">
      <panel header="Translate" toggleable>
        <div class="panel">
          Target language
          <dropdown v-model="config.translate.language" :options="languages"
                    option-label="name" option-value="code"/>
        </div>
      </panel>
      <panel header="Image List" toggleable>
        <div class="panel">
          Image width
          <div style="display: flex; align-items: center; gap: 1em">
            <label>{{config.imageList.width}} px</label>
            <Slider v-model="config.imageList.width" style="width: 14rem"
                    :min="50" :max="300" :step="10" />
          </div>
        </div>
      </panel>
    </div>
  </div>
</template>

<style scoped>
.settings-window {
  max-width: 750px;
  min-width: 500px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.settings-header {
  margin: 0 8px;
}

.settings-panel {
  height: 100%;
  overflow-y: auto;
}

.settings-panel > * {
  margin: 8px;
}

h2 {
  color: var(--text-color);
}

i {
  padding: 2px;
  font-size: 1.2em;
  border-radius: 1em;
  transition: background-color 0.25s;
}

i:hover {
  cursor: pointer;
  background-color: var(--surface-300);
}

.panel {
  display: flex;
  justify-content: space-between;
}
</style>
