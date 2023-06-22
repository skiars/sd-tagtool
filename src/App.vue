<script setup lang="ts">
import {ref} from 'vue'

import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel'

import MenuBar from './components/MenuBar.vue'
import ImageList from './components/ImageList.vue'
import TagList from './components/TagList.vue'
import {Menu, TagData} from './lib/types'
import {TagEditor} from "./lib/history";

import {open} from '@tauri-apps/api/dialog'
import {invoke} from "@tauri-apps/api/tauri"
import {join} from '@tauri-apps/api/path'
import {convertFileSrc} from '@tauri-apps/api/tauri'
import {exit} from '@tauri-apps/api/process';

let tagEditor: TagEditor
const workDir = ref<string>('')
const dataset = ref<TagData[]>([])
const tags = ref<string[]>([])
const selected = ref<number[]>([])

async function openDir(path: string) {
  // const selected = await open({ directory: true }) as string
  const files: {
    name: string,
    tags: string[]
  }[] = await invoke("listdir", {path: path})
  const data: TagData[] = []
  for (let i in files)
    data.push({
      name: files[i].name,
      url: convertFileSrc(await join(path, files[i].name)),
      tags: files[i].tags
    })
  workDir.value = path
  dataset.value = data
  tagEditor = new TagEditor(dataset.value)
}

openDir('E:/diffusion/dataset/reg')

function selectedTags(d: { index: number }[]) {
  selected.value = d.map(x => x.index)
  tags.value = dataset.value[d[0].index].tags
}

function onTagsChange(x: string[]) {
  if (selected.value.length == 1)
    tagEditor.edit([{index: selected.value[0], tags: x}])
}

async function onMenuAction(action: Menu) {
  switch (action) {
    case Menu.Open:
      await openDir(await open({directory: true}) as string)
      break
    case Menu.Quit:
      await exit()
      break
    case Menu.Undo: {
      const d = tagEditor.undo()
      if (selected.value.length)
        tags.value = d[selected.value[0]].tags
      break
    }
    case Menu.Redo: {
      const d = tagEditor.redo()
      if (selected.value.length)
        tags.value = d[selected.value[0]].tags
      break
    }
  }
}
</script>

<template>
  <header>
    <MenuBar v-on:action="onMenuAction($event)"/>
  </header>
  <Splitter class="main-content">
    <SplitterPanel :size="20">
      <ImageList :dataset="dataset" v-on:select="selectedTags($event)"/>
    </SplitterPanel>
    <SplitterPanel :size="80">
      <TagList v-model="tags" v-on:update:modelValue="onTagsChange($event)"/>
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
}

.main-content {
  height: calc(100vh - 50px);
  background-color: transparent;
}
</style>
