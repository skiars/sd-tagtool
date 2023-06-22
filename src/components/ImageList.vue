<script setup lang="ts">
import {TagData} from '../lib/types'

const props = defineProps<{
  dataset: TagData[]
}>()

const emit = defineEmits<{
  select: { index: number }[]
}>()
</script>

<template>
  <div class="image-list">
    <div class="item" v-for="(item, index) in props.dataset"
         v-on:click="emit('select', [{ index: index }])">
      <img class="image" :src="item.url"/>
    </div>
  </div>
</template>

<style scoped>
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
