<script setup lang="ts">
import {ref} from 'vue'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import TagInput from './TagInput.vue'

const emit = defineEmits<{
  (e: 'filter', value: { tags: string[], exclude: boolean }): void
}>()

let tags: string[] = []
let exclude = ref(false)

</script>

<template>
  <div class="tag-input-container">
    <tag-input class="tag-input"
               placeholder="Enter tags and filter images"
               v-on:updateTags="x => tags = x"/>
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