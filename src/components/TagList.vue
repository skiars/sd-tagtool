<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import draggable from 'vuedraggable';
import Tag from './Tag.vue'

const props = defineProps<{
  modelValue: string[]
}>()

const emit = defineEmits(['update:modelValue'])

const drag = ref(false)

const tags = computed({
  get() { return props.modelValue },
  set(v: string[]) { emit('update:modelValue', v) }
})
</script>

<template>
  <draggable class="tag-list" v-model="tags" :item-key="(x: string) => x" ghost-class="ghost" :animation="200"
    :component-data="{
      tag: 'div',
      type: 'transition-group',
      name: !drag ? 'flip-list' : null
    }" @start="drag = true" @end="drag = false">
    <template #item="{ element, index }">
      <Tag class="list-group-item tag-item" removable :label="element" v-on:remove="tags = tags.splice(index, 1)" />
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

.flip-list-move {
  transition: transform 0.2s;
}
</style>
