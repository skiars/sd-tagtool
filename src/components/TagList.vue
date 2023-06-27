<script setup lang="ts">
import {ref, shallowRef, watch} from 'vue'
import draggable from 'vuedraggable';
import Tag from './Tag.vue'
import ContextMenu from 'primevue/contextmenu'
import * as state from "../lib/state";

const props = defineProps<{
  tags: string[]
  nodrag?: true | boolean
  editable?: true | boolean
}>()

const tags = ref<string[]>([])
const contentTag = ref<string>('')
const menu = shallowRef()

watch(() => props.tags, x => tags.value = x)

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
      emit('filter', [contentTag.value])
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

function onRightClick(label: string, event: UIEvent) {
  contentTag.value = label
  menu.value.show(event)
}
</script>

<template>
  <draggable class="tag-list"
             v-model="tags"
             :disabled="props.nodrag"
             :item-key="(x: string) => x"
             ghost-class="ghost"
             :animation="200"
             v-on:end="emit('sorted', tags)">
    <template #item="{ element }">
      <Tag class="list-group-item tag-item" :style="tagStyle(element)"
           :label="element" :removable="props.editable"
           v-on:delete="emit('delete', [element])"
           v-on:dblclick="emit('active', [element])"
           v-on:contextmenu="e => onRightClick(element, e)"/>
    </template>
    <template #footer>
      <context-menu ref="menu" :model="contentMenu"/>
    </template>
  </draggable>
</template>

<style scoped>
.tag-list {
  min-height: 5em;
  display: flex;
  flex-wrap: wrap;
  overflow-y: auto;
  align-items: baseline;
  align-content: start;
  border: 1px solid var(--gray-300);
  border-radius: var(--border-radius);
  background-color: var(--surface-card);
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
