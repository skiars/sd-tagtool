<script setup lang="ts">
import {computed, ref, onMounted, watch} from 'vue'
import 'primeicons/primeicons.css'
import {invoke} from '@tauri-apps/api/tauri'
import * as state from '../lib/state'

const props = defineProps<{
  label: string,
  removable?: true | boolean
}>()

const emit = defineEmits<{
  (e: 'delete'): void
}>()

const trLabel = ref<string>()
const tr = computed<boolean>(() => Boolean(trLabel.value) && trLabel.value != props.label)

onMounted(async () => {
  if (state.translate.value)
    trLabel.value = await invoke('translate_tag', {text: props.label}) as string
})

watch(() => state.translate.value, async enable => {
  trLabel.value = enable
    ? await invoke('translate_tag', {text: props.label}) as string
    : undefined
})
</script>

<template>
  <div class="tag-frame">
    <span class="tag-label">
      <span>{{ props.label }}</span>
      <span class="translate-label" v-if="tr">&nbsp;{{ trLabel }} </span>
    </span>
    <i v-if="removable" class="pi pi-times" v-on:click="emit('delete')"/>
  </div>
</template>

<style scoped>
.tag-frame {
  display: flex;
  padding-left: 0.25em;
  padding-right: 0.25em;
  align-items: center;
  color: var(--primary-color-text);
  background-color: var(--primary-400);
  border-radius: 0.5em;
  cursor: default;
  -webkit-user-select: none; /* Safari */
  -ms-user-select: none; /* IE 10 and IE 11 */
  user-select: none; /* Standard syntax */
}

.tag-label {
  margin: 0.2em 0.1em 0.2em 0.1em;
}

.translate-label {
  opacity: 0.75;
}

i {
  font-size: 0.85em;
  padding: 0.1em;
  border-radius: 1em;
}

i:hover {
  cursor: pointer;
}
</style>
