<script setup lang="ts">
import {ref, shallowRef, watch, onUnmounted, onMounted} from 'vue'
import draggable from 'vuedraggable';
import Tag from './Tag.vue'
import ContextMenu from 'primevue/contextmenu'
import Toast from 'primevue/toast'
import {useToast} from 'primevue/usetoast'
import * as state from '../lib/state'

const props = defineProps<{
  tags: string[]
  nodrag?: true | boolean
  editable?: true | boolean,
  highlight?: string[]
}>()

const tags = ref<string[]>([])
const contentTag = ref<string>('')
const selectTags = ref<Set<number>>(new Set)
const highlightTags = ref<Set<string>>(new Set)
const menu = shallowRef()
let pressCtrl = false, pressShift = false
let selected = -1
const toast = useToast();

const emit = defineEmits<{
  (e: 'sorted', value: string[]): void
  (e: 'delete', value: string[]): void
  (e: 'active', value: string[]): void
  (e: 'filter', value: string[]): void
}>()

const paletteColors = [
  '#435b66', '#867070', '#a76f6f', '#eab2a0',
  '#ffb100', '#e74646', '#b71375', '#a459d1',
  '#6930c3', '#007ea7', '#25a18e', '#7fb77e'
]

const contentMenu = ref([
  {
    label: 'Add filter',
    command() {
      emit('filter', contentTags())
    }
  },
  {
    label: 'Copy tag(s)',
    command() {
      navigator.clipboard.writeText(contentTagsText())
      toast.add({severity: 'success', detail: 'Tags has been copied.', life: 500})
    }
  },
  {
    label: 'Pick color',
    class: 'tag-palette-menu',
    items: setupPalette()
  },
  {
    label: 'Clear picked color',
    command() {
      state.tagPalette.value.delete(contentTag.value)
    }
  }
])

watch(() => props.tags, x => {
  tags.value = x
  resetSelect()
})

watch(() => props.highlight, x => {
  highlightTags.value = new Set(x)
})

function setupPalette(): object[] {
  return paletteColors.map(c => ({
    class: 'tag-palette-item',
    style: {backgroundColor: c},
    command() {
      state.tagPalette.value.set(contentTag.value, c)
    }
  }))
}

function tagStyle(tag: string): {} | undefined {
  const pal = state.tagPalette.value.get(tag)
  if (props.nodrag && !pal)
    return undefined
  let s: any = {}
  if (!props.nodrag)
    s.cursor = 'move'
  if (pal)
    s.backgroundColor = pal
  return s
}

function contentTags(): string[] {
  if (selectTags.value.size)
    return Array.from(selectTags.value.keys()).sort().map(x => tags.value[x])
  return [contentTag.value]
}

function contentTagsText(): string {
  return contentTags().join(',')
}

function resetSelect() {
  selectTags.value.clear()
  selected = -1
}

function onClick(index: number) {
  if (pressCtrl) {
    selected = index
    if (selectTags.value.has(index))
      selectTags.value.delete(index)
    else
      selectTags.value.add(index)
  } else if (pressShift && selected >= 0) {
    let a = Math.min(selected, index)
    let b = Math.max(selected, index)
    selectTags.value.clear()
    for (; a <= b; a++)
      selectTags.value.add(a)
  } else {
    selected = index
    selectTags.value.clear()
    selectTags.value.add(index)
  }
}

function onRightClick(index: number, event: UIEvent) {
  if (!selectTags.value.has(index))
    resetSelect()
  contentTag.value = tags.value[index]
  menu.value.show(event)
}

function onKeyDown(event: KeyboardEvent) {
  if (event.key == 'Control')
    pressCtrl = true
  if (event.key == 'Shift')
    pressShift = true
}

function onKeyUp(event: KeyboardEvent) {
  if (event.key == 'Control')
    pressCtrl = false
  if (event.key == 'Shift')
    pressShift = false
}

onMounted(() => {
  window.addEventListener("keydown", onKeyDown)
  window.addEventListener("keyup", onKeyUp)
})

onUnmounted(() => {
  window.removeEventListener("keydown", onKeyDown)
  window.removeEventListener("keyup", onKeyUp)
})
</script>

<template>
  <div class="frame" :tabindex="-1">
    <draggable class="tag-list"
               v-model="tags"
               :disabled="props.nodrag"
               :item-key="(x: string) => x"
               ghost-class="ghost"
               :animation="200"
               v-on:end="emit('sorted', tags)"
               v-on:click.self="selectTags.clear()">
      <template #item="{ element, index }">
        <Tag class="list-group-item tag-item" :style="tagStyle(element)"
             :label="element" :removable="props.editable"
             :select="selectTags.has(index)"
             :highlight="highlightTags.has(element)"
             v-on:delete="emit('delete', [element])"
             v-on:dblclick="emit('active', [element])"
             v-on:click="onClick(index)"
             v-on:contextmenu="(e: UIEvent) => onRightClick(index, e)"/>
      </template>
    </draggable>
    <context-menu ref="menu" :model="contentMenu"/>
    <Toast/>
  </div>
</template>

<style scoped>
.frame {
  min-height: 4em;
  overflow-y: auto;
  border: 1px solid var(--gray-300);
  border-radius: var(--border-radius);
  background-color: var(--surface-card);
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  align-content: start;
}

.tag-item {
  margin: 4px 4px;
}
</style>

<style>
.tag-palette-item {
  width: 1.5em;
  height: 1.5em;
  border-radius: 0.3em;
  margin: 0.25em;
}

.tag-palette-menu .p-submenu-list {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  min-width: 0 !important;
  width: calc((1.5em + 0.25em) * 7) !important;
}

.tag-palette-menu .p-focus > .p-menuitem-content {
  background: transparent !important;
}

.tag-palette-menu .p-menuitem-content:hover {
  color: transparent !important;
  background: transparent !important;
}
</style>
