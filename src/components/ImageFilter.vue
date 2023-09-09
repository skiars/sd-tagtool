<script setup lang="ts">
import {ref, computed} from 'vue'
import Button from 'primevue/button'
import Dropdown from 'primevue/dropdown'
import TagInput from './TagInput.vue'
import {FilterMode} from '../lib/utils'

const props = defineProps<{
  modelValue: string[],
  suggestions: string[]
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string[]): void
  (e: 'filter', value: { tags: string[], mode: FilterMode }): void
}>()

const filterMode = ref(FilterMode.IncludeAny)
const tags = computed({
  get() {
    return props.modelValue
  },
  set(x: string[]) {
    emit('update:modelValue', x)
  }
})

const filterOptions = [
  {name: 'include any', value: FilterMode.IncludeAny},
  {name: 'include all', value: FilterMode.IncludeAll},
  {name: 'exclude', value: FilterMode.Exclude},
]
</script>

<template>
  <div class="tag-input-container">
    <tag-input v-model="tags" :suggestions="suggestions"
               placeholder="Enter tags and filter images"/>
    <dropdown class="filter-dropdown"
              v-model="filterMode" :options="filterOptions"
              optionLabel="name" optionValue="value"/>
    <Button class="fixed" rounded v-on:click="emit('filter', {tags: tags, mode: filterMode})">
      Filter
    </Button>
  </div>
</template>

<style scoped>
.tag-input-container {
  display: flex;
  align-items: center;
  gap: 0.5em;
}

.filter-dropdown {
  width: 7.5em;
}

.fixed {
  flex-shrink: 0;
  align-self: flex-start;
}

label {
  -webkit-user-select: none; /* Safari */
  -ms-user-select: none; /* IE 10 and IE 11 */
  user-select: none; /* Standard syntax */
}
</style>