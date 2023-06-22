<script setup lang="ts">
import {computed} from 'vue'
import draggable from 'vuedraggable';
import Tag from './Tag.vue'

const props = defineProps<{
  modelValue: string[]
}>()

const emit = defineEmits(['update:modelValue'])

const tags = computed({
  get() {
    return props.modelValue
  },
  set(v: string[]) {
    emit('update:modelValue', v)
  }
})

</script>

<template>
  <draggable class="tag-list"
             v-model="tags"
             :item-key="(x: string) => x"
             ghost-class="ghost"
             :animation="200">
    <template #item="{ element, index }">
      <Tag class="list-group-item tag-item" removable :label="element"
           v-on:remove="tags = tags.filter((_, i) => i != index)"/>
    </template>
  </draggable>
</template>

<style scoped>
.tag-list {
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
  margin: 4px 6px;
}
</style>
