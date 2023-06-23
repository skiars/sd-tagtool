<script setup lang="ts">
import 'primeicons/primeicons.css'

const props = defineProps<{
  label: string,
  removable?: true | boolean
}>()

const emit = defineEmits<{
  (e: 'delete'): void
}>()
</script>

<template>
  <div class="tag-frame">
    <span>{{ props.label }}</span>
    <i v-if="removable" class="pi pi-times"
       v-on:click="emit('delete')"></i>
  </div>
</template>

<style scoped>
.tag-frame {
  display: flex;
  padding-left: 0.25em;
  padding-right: 0.25em;
  align-items: center;
  color: var(--primary-color-text);
  background-color: var(--primary-400);
  border-radius: 0.5em;
  cursor: default;
  -webkit-user-select: none; /* Safari */
  -ms-user-select: none; /* IE 10 and IE 11 */
  user-select: none; /* Standard syntax */
}

span {
  margin: 0.2em 0.1em 0.2em 0.1em;
}

i {
  font-size: 0.875em;
  padding: 0.1em;
  border-radius: 1em;
}

i:hover {
  cursor: pointer;
}
</style>
