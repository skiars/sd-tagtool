<script setup lang="ts">
import {ref} from 'vue'

import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel'

import MenuBar from './components/MenuBar.vue'
import ImageList from './components/ImageList.vue'
import TagList from './components/TagList.vue'
import TagInput from "./components/TagInput.vue";
import {Menu, TagData} from './lib/types'
import {TagEditor, collectTags} from './lib/utils'

import {open} from '@tauri-apps/api/dialog'
import {invoke} from "@tauri-apps/api/tauri"
import {join} from '@tauri-apps/api/path'
import {convertFileSrc} from '@tauri-apps/api/tauri'
import {exit} from '@tauri-apps/api/process'

let tagEditor: TagEditor
const workDir = ref<string>('')
const dataset = ref<TagData[]>([])
const selected = ref<number[]>([])
const tags = ref(collectTags())
const allTags = ref(collectTags())

async function openDir(path: string) {
  // const selected = await open({ directory: true }) as string
  const files: {
    name: string,
    tags: string[]
  }[] = await invoke("listdir", {path: path})
  const data: TagData[] = []
  for (let i in files) {
    const v = files[i]
    data.push({
      key: parseInt(i),
      name: v.name,
      url: convertFileSrc(await join(path, v.name)),
      tags: v.tags
    })
  }
  tags.value = collectTags()
  allTags.value = collectTags(data)
  workDir.value = path
  dataset.value = data
  tagEditor = new TagEditor(dataset.value)
}

openDir('E:/diffusion/dataset/reg')

function selectedTags(d: { index: number }[]) {
  tags.value = collectTags(d.map(x => dataset.value[x.index]))
}

function onTagsChange(x: string[]) {
  console.log(x)
  if (selected.value.length == 1)
    tagEditor.edit([{index: selected.value[0], tags: x}])
}

async function onMenuAction(action: Menu) {
  switch (action) {
    case Menu.Open:
      await openDir(await open({directory: true}) as string)
      break
    case Menu.Quit:
      await exit(0)
      break
    case Menu.Undo: {
      const d = tagEditor.undo()
      tags.value = collectTags(selected.value.map(x => d[x]))
      break
    }
    case Menu.Redo: {
      const d = tagEditor.redo()
      tags.value = collectTags(selected.value.map(x => d[x]))
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
      <Splitter layout="vertical">
        <SplitterPanel class="column-flex">
          <TagList style="flex-grow: 1" :tags="tags.tags" v-on:change="onTagsChange($event)"/>
          <TagInput style="flex-shrink: 0"></TagInput>
        </SplitterPanel>
        <SplitterPanel class="column-flex">
          <TagList style="flex-grow: 1" :tags="allTags.tags" v-on:change="onTagsChange($event)"/>
        </SplitterPanel>
      </Splitter>
    </SplitterPanel>
  </Splitter>
  <footer>Footer</footer>
</template>

<style scoped>
header,
footer {
  margin-top: 0.25rem;
  margin-bottom: 0.25rem;
}

.main-content {
  height: calc(100vh - 100px);
  background-color: transparent;
}

.column-flex {
  display: flex;
  flex-direction: column;
  overflow-y: hidden;
}
</style>
