<script setup lang="ts">
import {ref, onMounted} from 'vue'
import {invoke} from '@tauri-apps/api/tauri'
import AutoComplete, {AutoCompleteCompleteEvent} from "primevue/autocomplete";

const props = defineProps<{
  translate?: true | boolean
  placeholder?: string
}>()

const emit = defineEmits<{
  (e: 'updateTags', value: string[]): void
}>()

interface TagHint {
  tag: string
  suggest?: string
  usage_count?: number
  translate?: string
}

const tags = ref<TagHint[]>([])
const suggestions = ref<TagHint[]>([]);

function optionLabel(s: TagHint) {
  return s.suggest ? s.suggest : s.tag
}

async function search(event: AutoCompleteCompleteEvent) {
  let parsed: string[] = await invoke('parse_tags', {text: event.query})
  if (parsed.length) {
    const unfinished = /,\s*$/.test(event.query) ? '' : parsed.pop()
    if (unfinished) {
      suggestions.value = await invoke('query_tag', {text: unfinished})
    }
    let target = event.originalEvent.target as HTMLInputElement
    target.value = unfinished ? unfinished : '';
    tags.value = tags.value.concat(parsed.map(x => ({tag: x})))
    if (unfinished && !suggestions.value.some(x => x.tag == unfinished))
      suggestions.value.unshift({tag: unfinished})
    if (props.translate) {
      suggestions.value.forEach(x => {
        invoke('translate_tag', {text: x.tag})
            .then(tr => x.translate = tr as string)
      })
    }
  } else {
    suggestions.value = []
  }
}

function readableNumber(x: number): string {
  return x < 1000 ? x.toString() : (x * 1e-3).toFixed(0) + 'k'
}
</script>

<template>
  <auto-complete v-model="tags" multiple :suggestions="suggestions"
                 :option-label="optionLabel" v-on:complete="search"
                 v-on:change="emit('updateTags', tags.map(optionLabel))"
                 :placeholder="!tags.length ? placeholder : ''">
    <template #option="{option}: {option: TagHint}">
      <span>{{ option.tag }}</span>
      <span v-if="option.suggest">&nbsp;→ {{ option.suggest }}</span>
      <span v-if="option.usage_count">&nbsp;({{ readableNumber(option.usage_count) }})</span>
      <span v-if="option.translate" class="translate-text">&nbsp;{{ option.translate }}</span>
    </template>
  </auto-complete>
</template>

<style scoped>
.translate-text {
  opacity: 0.75;
}
</style>