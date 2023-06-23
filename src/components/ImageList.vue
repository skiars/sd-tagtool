<script setup lang="ts">
import {ref, watch} from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import {TagData} from '../lib/types'

const props = defineProps<{
  dataset: TagData[]
}>()

const selected = ref<TagData[]>([])

const emit = defineEmits<{
  (e: 'select', value: { index: number }[]): void
  (e: 'openFolder'): void
}>()

watch(selected, value => {
  emit('select', value.map(x => ({index: x.key})))
})
</script>

<template>
  <DataTable :value="props.dataset"
             v-model:selection="selected" selection-mode="multiple" class="image-list">
    <template #empty>
      <div class="empty-list" v-on:click="emit('openFolder')">
        Open a folder and continue.
      </div>
    </template>
    <Column field="url">
      <template #body="{ data }">
        <div>
          <img class="image" :src="data.url" :alt="data.url"/>
        </div>
      </template>
    </Column>
  </DataTable>
</template>

<style scoped>
.empty-list {
  cursor: pointer;
  text-align: center;
}

.image-list {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.item {
  height: 20%;
  margin: 4px;
  padding: 4px;
  border-radius: 5px;
  background-color: var(--surface-card);
}

.image {
  width: auto;
  height: 15vh;
  width: 100%;
  height: 100%;
  object-fit: scale-down;
  text-align: center;
}
</style>

<style>
.image-list table thead {
  display: none;
}
</style>
