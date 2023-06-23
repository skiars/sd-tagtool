<script setup lang="ts">
import {ref, watch} from 'vue'
import draggable from 'vuedraggable';
import Tag from './Tag.vue'

const props = defineProps<{
  tags: string[]
  nodrag?: true | boolean
  translate?: true | boolean
  editable?: true | boolean
}>()

const tags = ref<string[]>([])
watch(() => props.tags, x => tags.value = x)

const emit = defineEmits<{
  (e: 'sorted', value: string[]): void
  (e: 'delete', value: string[]): void
  (e: 'active', value: string[]): void
}>()
</script>

<template>
  <draggable class="tag-list"
             v-model="tags"
             :disabled="nodrag"
             :item-key="(x: string) => x"
             ghost-class="ghost"
             :animation="200"
             v-on:end="emit('sorted', tags)">
    <template #item="{ element }">
      <Tag class="list-group-item tag-item" :style="nodrag ? null : 'cursor: move'"
           :label="element" :removable="props.editable" :translate="props.translate"
           v-on:delete="emit('delete', [element])"
           v-on:dblclick="emit('active', [element])"/>
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
