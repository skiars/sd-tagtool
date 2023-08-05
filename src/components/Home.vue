<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from 'vue-router'

import Splitter from 'primevue/splitter'
import SplitterPanel from 'primevue/splitterpanel'

import ImageList from './ImageList.vue'
import TagList from './TagList.vue'
import TagEditor from './TagEditor.vue'
import ImageFilter from './ImageFilter.vue'
import {TagData} from '../lib/types'
import {CollectTags, EditorHistory, collectTags, deleteTags, insertTags, FilterMode} from '../lib/utils'
import * as state from '../lib/state'

import {open} from '@tauri-apps/api/dialog'
import {invoke} from '@tauri-apps/api/tauri'
import {listen} from '@tauri-apps/api/event'
import {join} from '@tauri-apps/api/path'
import {convertFileSrc} from '@tauri-apps/api/tauri'
import {platform} from '@tauri-apps/api/os'
import {exit} from '@tauri-apps/api/process'
import {appWindow} from '@tauri-apps/api/window'
import {confirm, message} from '@tauri-apps/api/dialog'

let history: EditorHistory = new EditorHistory
let tagInsPos: number | undefined = undefined
let workDir: string = ''
let editState: Array<any> | undefined = undefined
const dataset = ref<TagData[]>([])
const filteredDataset = ref<TagData[]>([])
const tagsFilter = ref<string[]>([])
const selected = ref<number[]>([])
const selTags = ref(collectTags())
const allTags = ref(collectTags())
const editAllTags = ref(false)
const router = useRouter()

async function openFolder(path?: string) {
  if (!path) {
    const msg = 'The work has not been saved, are you sure to open a new folder?'
    if (editState != history.state() &&
      !await confirm(msg, {title: 'Confirm open folder', type: 'warning'}))
      return
    const result = await open({directory: true})
    if (!result)
      return
    path = result as string
  }
  const files: {
    name: string,
    tags: string[]
  }[] = await invoke('listdir', {path: path})
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
  workDir = path
  dataset.value = data
  filteredDataset.value = data
  selected.value = []
  updateTags(data)
  history = new EditorHistory(dataset.value)
  editState = undefined
  await appWindow.setTitle(`sd-tagtool - ${path}`)
}

async function quitApp() {
  const msg = 'The work has not been saved, are you sure to quit?'
  if (editState == history.state()
    || await confirm(msg, {title: 'Quit without save', type: 'warning'}))
    await exit(0)
}

appWindow.listen('tauri://close-requested', quitApp)

function selectedTags(d: { index: number }[]) {
  selected.value = d.map(x => x.index)
  selTags.value = collectTags(d.map(x => dataset.value[x.index]))
}

function onTagsChange(x: string[]) {
  if (selected.value.length == 1) {
    const d = history.edit([{index: selected.value[0], tags: x}])
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
  updateTags(history.edit(edit))
}

function onInsertTags(tags: string[]) {
  const sel: Set<number> = new Set(selected.value)
  const data = dataset.value.filter(x => sel.has(x.key))
  const edit = insertTags(data, tags, tagInsPos)
  updateTags(history.edit(edit))
}

function onAddTagFilter(e: string[]) {
  const set = new Set(tagsFilter.value)
  tagsFilter.value = tagsFilter.value.concat(e.filter(x => !set.has(x)))
}

function onFilterApply(e: { tags: string[], mode: FilterMode }) {
  if (e.tags) {
    function includeAny(x: TagData): boolean {
      const s = new Set(x.tags)
      return e.tags.some(a => s.has(a))
    }

    let filter: (x: TagData) => boolean
    switch (e.mode) {
      case FilterMode.IncludeAny:
        filter = includeAny
        break
      case FilterMode.IncludeAll:
        filter = x => {
          const s = new Set(x.tags)
          return e.tags.every(a => s.has(a))
        }
        break;
      case FilterMode.Exclude:
        filter = x => !includeAny(x)
        break
    }
    filteredDataset.value = dataset.value.filter(filter)
  } else {
    filteredDataset.value = dataset.value
  }
  selected.value = []
  selTags.value = collectTags()
}

async function menuAction(menu: string) {
  if (router.currentRoute.value.path != '/' && menu != 'quit') {
    await message('This action is only available on the home page!',
      {title: 'Invalid operation', type: 'warning'})
    return
  }
  switch (menu) {
    case 'open':
      await openFolder()
      break
    case 'save':
      await Promise.all(dataset.value.map(async x => {
        await invoke('save_tags', {
          path: await join(workDir, x.name),
          text: x.tags.join(', ')
        })
      }))
      editState = history.state()
      if (dataset.value.length)
        await message('All content has been saved!', 'Save')
      break
    case 'quit':
      await quitApp()
      break
    case 'undo':
      updateTags(history.undo())
      break
    case 'redo':
      updateTags(history.redo())
      break
    case 'settings':
      router.push('/settings').then()
  }
}

listen('menu', async event => menuAction(event.payload as string))
platform().then(name => {
  if (name == 'win32') { // Menu shortcuts registered in the backend on Windows do not work...
    document.addEventListener('keydown', e => {
      function handle(action: string, key: string, shift: boolean = false) {
        if (e.ctrlKey && !e.altKey && e.shiftKey == shift && e.code == key)
          menuAction(action)
      }

      handle('open', 'KeyO')
      handle('save', 'KeyS')
      handle('undo', 'KeyZ')
      handle('redo', 'KeyZ', true)
    }, false);
  }
})

listen('translate', event => {
  state.translate.value = event.payload as boolean
})

// load tags database
const date = Date.now()
invoke('load_tags_db', {}).then(() => console.log(`load tags db finished ${Date.now() - date} ms.`))
</script>

<template>
  <splitter class="main-content">
    <splitter-panel :size="20">
      <image-list :dataset="filteredDataset" v-on:select="selectedTags"/>
    </splitter-panel>
    <splitter-panel :size="80">
      <splitter layout="vertical">
        <splitter-panel class="column-flex">
          <image-filter v-model="tagsFilter"
                        :suggestions="allTags.tags"
                        v-on:filter="onFilterApply"/>
          <tag-list style="flex-grow: 1" :tags="selTags.tags"
                    editable :nodrag="selected.length > 1"
                    v-on:sorted="onTagsChange"
                    v-on:delete="x => onDeleteTags(selTags, x)"
                    v-on:filter="onAddTagFilter"/>
          <tag-editor style="flex-shrink: 0"
                      v-model:editAllTags="editAllTags"
                      v-on:updatePosition="x => tagInsPos = x"
                      v-on:updateTags="onInsertTags"/>
        </splitter-panel>
        <splitter-panel class="column-flex">
          <tag-list style="flex-grow: 1" :tags="allTags.tags"
                    :editable="editAllTags" nodrag
                    v-on:delete="e => onDeleteTags(allTags, e)"
                    v-on:active="onInsertTags"
                    v-on:filter="onAddTagFilter"/>
        </splitter-panel>
      </splitter>
    </splitter-panel>
  </splitter>
</template>

<style scoped>
header,
footer {
  margin-top: 0.25rem;
  margin-bottom: 0.25rem;
}

.main-content {
  height: calc(100vh - 20px);
  background-color: transparent;
}

.column-flex {
  padding: 0.5em;
  display: flex;
  flex-direction: column;
  gap: 0.5em;
  overflow-y: clip;
}
</style>
