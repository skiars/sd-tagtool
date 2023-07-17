<script setup lang="ts">
import {ref, computed} from 'vue'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import TagInput from './TagInput.vue'

const props = defineProps<{
  modelValue: string[],
  suggestions: string[]
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string[]): void
  (e: 'filter', value: { tags: string[], exclude: boolean }): void
}>()

const exclude = ref(false)
const tags = computed({
  get() {
    return props.modelValue
  },
  set(x: string[]) {
    emit('update:modelValue', x)
  }
})
</script>

<template>
  <div class="tag-input-container">
    <tag-input class="tag-input" v-model="tags" :suggestions="suggestions"
               placeholder="Enter tags and filter images"/>
    <checkbox v-model="exclude" :binary="true" inputId="image-filter-checkbox"/>
    <label for="image-filter-checkbox">exclude</label>
    <Button rounded v-on:click="emit('filter', {tags: tags, exclude: exclude})">
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

.tag-input {
  flex-grow: 1;
}

label {
  -webkit-user-select: none; /* Safari */
  -ms-user-select: none; /* IE 10 and IE 11 */
  user-select: none; /* Standard syntax */
}
</style>