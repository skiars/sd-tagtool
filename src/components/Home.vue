<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRouter} from 'vue-router'

import Splitter from 'primevue/splitter'
import SplitterPanel from 'primevue/splitterpanel'

import ImageList from './ImageList.vue'
import TagList from './TagList.vue'
import TagEditor from './TagEditor.vue'
import ImageFilter from './ImageFilter.vue'
import {TagData, EditorHistory, collectTags, deleteTags, insertTags, FilterMode, replaceTags} from '../lib/utils'
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
import DeleteIsolatedTxt from './DeleteIsolatedTxt.vue'
import {deleteIsolatedTxt} from './DeleteIsolatedTxt'

let history: EditorHistory = new EditorHistory
let workDir: string = ''
let editState: Array<any> | undefined = undefined
const dataset = ref<TagData[]>([])
const filteredDataset = ref<TagData[]>([])
const tagsFilter = ref<string[]>([])
const selected = ref<number[]>([])
const editAllTags = ref(false)
const router = useRouter()

const selTags = computed(() => collectTags(selectedDataset()))
const allTags = computed(() => collectTags(dataset.value).sort())

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

function selectedDataset(): TagData[] {
  const sel: Set<number> = new Set(selected.value)
  return dataset.value.filter(x => sel.has(x.key))
}

function editableDataset(): TagData[] {
  return editAllTags.value ? dataset.value : selectedDataset()
}

function onTagsChange(x: string[]) {
  if (selected.value.length == 1)
    history.edit([{index: selected.value[0], tags: x}])
}

function onDeleteTags(d: TagData[], tags: string[]) {
  history.edit(deleteTags(d, tags))
}

function onInsertTags({tags, position}: { tags: string[], position?: number }) {
  history.edit(insertTags(editableDataset(), tags, position))
}

function onReplaceTags({from, to}: { from: string[], to: string[] }) {
  history.edit(replaceTags(editableDataset(), from, to))
}

function onAddTagFilter(e: string[]) {
  const set = new Set(tagsFilter.value)
  tagsFilter.value = tagsFilter.value.concat(e.filter(x => !set.has(x)))
}

function onFilterApply(e: { tags: string[], mode: FilterMode }) {
  if (e.tags.length) {
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
    case 'reload':
      const msg = 'The work has not been saved, are you sure to reload folder?'
      if (editState == history.state() ||
        await confirm(msg, {title: 'Confirm reload folder', type: 'warning'}))
        await openFolder(workDir)
      break
    case 'quit':
      await quitApp()
      break
    case 'undo':
      history.undo()
      break
    case 'redo':
      history.redo()
      break
    case 'settings':
      router.push('/settings').then()
      break
    case 'delete_txt':
      await deleteIsolatedTxt(workDir)
      break;
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
      handle('reload', 'KeyR')
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
      <image-list :dataset="filteredDataset"
                  v-on:select="e => selected = e.map(x => x.index)"/>
    </splitter-panel>
    <splitter-panel :size="80">
      <splitter layout="vertical">
        <splitter-panel class="column-flex">
          <image-filter v-model="tagsFilter"
                        :suggestions="allTags"
                        v-on:filter="onFilterApply"/>
          <tag-list style="flex-grow: 1" :tags="selTags"
                    editable :nodrag="selected.length > 1"
                    v-on:sorted="onTagsChange"
                    v-on:delete="e => onDeleteTags(selectedDataset(), e)"
                    v-on:filter="onAddTagFilter"/>
          <tag-editor style="flex-shrink: 0"
                      v-model:editAllTags="editAllTags"
                      v-on:insertTags="onInsertTags"
                      v-on:replaceTags="onReplaceTags"/>
        </splitter-panel>
        <splitter-panel class="column-flex">
          <tag-list style="flex-grow: 1" :tags="allTags"
                    :editable="editAllTags" nodrag
                    v-on:delete="e => onDeleteTags(dataset, e)"
                    v-on:active="e => onInsertTags({tags: e})"
                    v-on:filter="onAddTagFilter"/>
        </splitter-panel>
      </splitter>
    </splitter-panel>
  </splitter>
  <delete-isolated-txt/>
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
