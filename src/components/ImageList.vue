<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref, shallowRef, watch} from 'vue'
import {listen} from '@tauri-apps/api/event'
import {TagData} from '../lib/utils'
import {config} from '../lib/state'

const props = defineProps<{
  dataset: TagData[]
}>()

const preview = ref(false)
const previewSrc = ref<string>()
const panel = shallowRef<HTMLElement>()
const overlay = shallowRef<HTMLElement>()
let enablePreview = false

const selectedSet = ref<Set<number>>(new Set)
let pressCtrl = false, pressShift = false
let selected = -1

const emit = defineEmits<{
  (e: 'select', value: { index: number }[]): void
}>()

watch(selectedSet, value => {
  emit('select', Array.from(value.keys()).sort()
    .map(x => ({index: props.dataset[x].key})))
}, {deep: true})

watch(() => props.dataset, x => {
  selectedSet.value.clear()
  selected = -1
})

listen('preview', event => {
  enablePreview = event.payload as boolean
})

const itemStyle = computed(() => ({
  maxWidth: `${config.value.imageList.width * 1.5}px`,
  flex: `1 1 ${config.value.imageList.width}px`
}))

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

let trigger: any = null

function onMouseEnter() {
  if (enablePreview) {
    trigger = setTimeout(() => {
      preview.value = true
      trigger = null
    }, 250)
  }
}

function onMouseLeave() {
  clearTimeout(trigger)
  trigger = null
  preview.value = false
}

function onClick(index: number) {
  if (pressCtrl) {
    selected = index
    if (selectedSet.value.has(index))
      selectedSet.value.delete(index)
    else
      selectedSet.value.add(index)
  } else if (pressShift && selected >= 0) {
    let a = Math.min(selected, index)
    let b = Math.max(selected, index)
    selectedSet.value.clear()
    for (; a <= b; a++)
      selectedSet.value.add(a)
  } else {
    selected = index
    selectedSet.value.clear()
    selectedSet.value.add(index)
  }
}

const resizeObserver = new ResizeObserver((event) => {
  updateOverlayPanelSize()
})

onMounted(() => {
  resizeObserver.observe(panel.value as HTMLElement)
  updateOverlayPanelSize()
  window.addEventListener("keydown", onKeyDown)
  window.addEventListener("keyup", onKeyUp)
})

onUnmounted(() => {
  resizeObserver.unobserve(panel.value as HTMLElement)
  window.removeEventListener("keydown", onKeyDown)
  window.removeEventListener("keyup", onKeyUp)
})

function updateOverlayPanelSize() {
  const el = overlay.value as HTMLElement
  const rootRect = document.documentElement.getBoundingClientRect()
  const rect = (panel.value as HTMLElement).getBoundingClientRect()
  const left = rect.right
  const width = rootRect.width - left - 16
  const height = rootRect.height - 20
  el.style.setProperty("top", `${0}px`)
  el.style.setProperty("left", `${left}px`)
  el.style.setProperty("width", `${width}px`)
  el.style.setProperty("height", `${height}px`)
}
</script>

<template>
  <div ref="panel" class="image-list" :tabindex="-1"
       v-on:mouseenter="onMouseEnter" v-on:mouseleave="onMouseLeave">
    <div v-for="(data, index) in props.dataset" :style="itemStyle"
         :class="['item', selectedSet.has(index) ? 'selected' : undefined]"
         v-on:mouseover="previewSrc = data.url" v-on:click="onClick(index)">
      <img class="image-thumb"
           :src="data.url" :alt="data.url"/>
    </div>
    <div v-if="!props.dataset.length" class="empty-list">There are no images to show.</div>
  </div>
  <Transition>
    <div v-show="previewSrc && preview" ref="overlay" class="overlay-panel">
      <img v-if="previewSrc" class="image-preview" alt="Preview"
           :src="previewSrc" :srcset="`${previewSrc} 2x`"/>
    </div>
  </Transition>
</template>

<style scoped>
.image-list {
  border: 10px;
  width: 100%;
  height: 100%;
  display: flex;
  flex-wrap: wrap;
  align-content: flex-start;
  align-items: stretch;
  overflow-y: auto;
  clip-path: inset(4px 0 4px 4px round 4px);
  padding: 2px 0 2px 2px;
}

.empty-list {
  text-align: center;
}

.item {
  margin: 2px;
  padding: 2px;
  border: 2px solid transparent;
  border-radius: 4px;
  background-color: var(--surface-card);
}

.item:hover {
  border: 2px solid var(--primary-400);
}

.selected {
  background-color: var(--blue-200);
}

.image-thumb {
  width: 100%;
  height: 100%;
  margin: auto;
  object-fit: scale-down;
  text-align: center;
}

.image-preview {
  object-fit: scale-down;
  text-align: center;
  max-width: 100%;
  max-height: 100%;
  padding: 8px;
  background-color: #ddd3;
  backdrop-filter: blur(10px);
  border-radius: 8px;
}

.overlay-panel {
  position: absolute;
  z-index: 1000;
  margin: 8px;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;
}

.v-enter-active,
.v-leave-active {
  transition: opacity 0.25s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>

<style>
.image-list table thead {
  display: none;
}
</style>
