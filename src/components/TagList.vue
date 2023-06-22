<script setup lang="ts">
import {ref, watch} from 'vue'
import draggable from 'vuedraggable';
import Tag from './Tag.vue'

const props = defineProps<{
  tags: string[]
}>()

const tags = ref<string[]>([])
watch(() => props.tags, x => tags.value = x)

const emit = defineEmits<{
  change: string[]
  remove: string[]
}>()
</script>

<template>
  <draggable class="tag-list"
             v-model="tags"
             :item-key="(x: string) => x"
             ghost-class="ghost"
             :animation="200"
             v-on:change="emit('change', $event)">
    <template #item="{ element }">
      <Tag class="list-group-item tag-item" removable :label="element"
           v-on:remove="emit('remove', [element])"/>
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
  margin: 4px 6px;
}
</style>
