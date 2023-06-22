<script setup lang="ts">
import {ref} from 'vue'

import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel'

import MenuBar from './components/MenuBar.vue'
import ImageList from './components/ImageList.vue'
import TagList from './components/TagList.vue'
import TagInput from "./components/TagInput.vue";
import {Menu, TagData} from './lib/types'
import {CollectTags, TagEditor, collectTags, deleteTags, insertTags} from './lib/utils'

import {open} from '@tauri-apps/api/dialog'
import {invoke} from "@tauri-apps/api/tauri"
import {join} from '@tauri-apps/api/path'
import {convertFileSrc} from '@tauri-apps/api/tauri'
import {exit} from '@tauri-apps/api/process'

let tagEditor: TagEditor
let tagInsPos: number | undefined = undefined
const workDir = ref<string>('')
const dataset = ref<TagData[]>([])
const selected = ref<number[]>([])
const selTags = ref(collectTags())
const allTags = ref(collectTags())
const editAllTags = ref(false)

async function openDir(path: string) {
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
  workDir.value = path
  dataset.value = data
  selected.value = []
  updateTags(data)
  tagEditor = new TagEditor(dataset.value)
}

openDir('E:/diffusion/dataset/reg')

function selectedTags(d: { index: number }[]) {
  selected.value = d.map(x => x.index)
  selTags.value = collectTags(d.map(x => dataset.value[x.index]))
}

function onTagsChange(x: string[]) {
  if (selected.value.length == 1) {
    const d = tagEditor.edit([{index: selected.value[0], tags: x}])
    updateTags(d)
  }
}

function updateTags(d: TagData[] | undefined) {
  if (d) {
    selTags.value = collectTags(selected.value.map(x => d[x]))
    allTags.value = collectTags(d)
    allTags.value.tags.sort()
  }
}

function onDeleteTags(collect: CollectTags, tags: string[]) {
  const edit = deleteTags(dataset.value, collect, tags)
  updateTags(tagEditor.edit(edit))
}

function onInsertTags(tags: string[]) {
  const sel: Set<number> = new Set(selected.value)
  const edit = insertTags(dataset.value.filter(x => sel.has(x.key)), tags, tagInsPos)
  updateTags(tagEditor.edit(edit))
}

async function onMenuAction(action: Menu) {
  switch (action) {
    case Menu.Open:
      await openDir(await open({directory: true}) as string)
      break
    case Menu.Quit:
      await exit(0)
      break
    case Menu.Undo:
      updateTags(tagEditor.undo())
      break
    case Menu.Redo:
      updateTags(tagEditor.redo())
      break
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
          <TagList style="flex-grow: 1" :tags="selTags.tags"
                   editable :nodrag="selected.length > 1"
                   v-on:sorted="onTagsChange($event)"
                   v-on:delete="onDeleteTags(selTags, $event)"/>
          <TagInput style="flex-shrink: 0"
                    v-model:editAllTags="editAllTags"
                    v-on:updatePosition="tagInsPos = $event"
                    v-on:updateText="onInsertTags($event)"/>
        </SplitterPanel>
        <SplitterPanel class="column-flex">
          <TagList style="flex-grow: 1" :tags="allTags.tags"
                   :editable="editAllTags" ondrag
                   v-on:delete="onDeleteTags(allTags, $event)"
                   v-on:active="onInsertTags($event)"/>
        </SplitterPanel>
      </Splitter>
    </SplitterPanel>
  </Splitter>
</template>

<style scoped>
header,
footer {
  margin-top: 0.25rem;
  margin-bottom: 0.25rem;
}

.main-content {
  height: calc(100vh - 70px);
  background-color: transparent;
}

.column-flex {
  display: flex;
  flex-direction: column;
  overflow-y: hidden;
}
</style>
