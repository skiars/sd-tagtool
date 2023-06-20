<script setup lang="ts">
import {ref} from 'vue'

import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';

import ImageList from './components/ImageList.vue'
import TagList from './components/TagList.vue';
import {TagData} from './types'

import {open} from '@tauri-apps/api/dialog'
import {invoke} from "@tauri-apps/api/tauri"
import {join} from '@tauri-apps/api/path'
import {convertFileSrc} from '@tauri-apps/api/tauri'

const workDir = ref<string>("")
const dataset = ref<TagData[]>([])

const tags = ref(['tree', 'solo', 'Action'])

async function openDir() {
  // const selected = await open({ directory: true }) as string
  const selected = 'E:/diffusion/dataset/reg'
  const files: {
    name: string,
    tags: string[]
  }[] = await invoke("listdir", {path: selected})
  const data: TagData[] = []
  for (let i in files)
    data.push({
      name: files[i].name,
      url: convertFileSrc(await join(selected, files[i].name)),
      tags: files[i].tags
    })
  workDir.value = selected
  dataset.value = data
}

openDir()

function selectedTags(d: TagData[]) {
  tags.value = d[0].tags
}
</script>

<template>
  <header>Header</header>
  <Splitter class="main-content">
    <SplitterPanel :size="20">
      <ImageList :dataset="dataset" v-on:select="selectedTags($event)"/>
    </SplitterPanel>
    <SplitterPanel :size="80">
      <TagList v-model="tags"></TagList>
      <div style="height: 50%"></div>
    </SplitterPanel>
  </Splitter>
  <footer>Footer</footer>
</template>

<style scoped>
header,
footer {
  margin: 4px;
  padding: 12px;
  text-transform: uppercase;
  color: #666;
}

.main-content {
  height: calc(100vh - 50px);
  background-color: transparent;
}
</style>
