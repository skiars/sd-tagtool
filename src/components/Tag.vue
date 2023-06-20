<script setup lang="ts">
import { ref } from 'vue'
import Chip from 'primevue/chip'
import 'primeicons/primeicons.css'

const props = defineProps<{
  label: string,
  removable?: true
}>()

const emit = defineEmits(['remove'])
</script>

<template>
  <div class="tag-frame">
    <span>{{ props.label }}</span>
    <i v-if="removable" class="pi pi-times" style="font-size: 0.5em" v-on:click="emit('remove')"></i>
  </div>
</template>

<style scoped>
.tag-frame {
  display: flex;
  align-items: baseline;
  padding-left: 0.5em;
  padding-right: 0.5em;
  color: var(--primary-color-text);
  background-color: var(--primary-400);
  border-radius: 0.5em;
  gap: 0.25em;
}

span {
  margin-top: 0.2em;
  margin-bottom: 0.2em;
}

i:hover {
  cursor: pointer;
}
</style>
