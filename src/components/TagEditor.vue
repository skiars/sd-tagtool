<script setup lang="ts">
import {ref, computed, watch} from 'vue'
import Button from 'primevue/button'
import InputNumber from 'primevue/inputnumber'
import InputSwitch from 'primevue/inputswitch'
import TagInput from './TagInput.vue'

const props = defineProps<{
  editAllTags: boolean,
  translate?: true | boolean
}>()

const emit = defineEmits<{
  (e: 'updateTags', value: string[]): void
  (e: 'updatePosition', value: number | undefined): void
  (e: 'update:editAllTags', value: boolean): void
}>()

const position = ref<number>()
let tags: string[] = []

const editAllTags = computed<boolean>({
  get() {
    return props.editAllTags
  },
  set(x) {
    emit('update:editAllTags', x)
  }
})
</script>

<template>
  <div class="tag-input-container">
    <span>position</span>
    <input-number v-model="position" placeholder="auto"
                  v-on:update:modelValue="emit('updatePosition', $event)"
                  :inputStyle="{ padding: '0.25em', width: '5em' }"/>
    <span>add tag</span>
    <tag-input class="tag-input" :translate="props.translate"
               placeholder="Separate tags with ',' or press Enter"
               v-on:updateTags="x => tags = x"/>
    <Button rounded v-on:click="emit('updateTags', tags)">Insert</Button>
    <span>edit all tags</span>
    <input-switch v-model="editAllTags"></input-switch>
  </div>
</template>

<style scoped>
.tag-input-container {
  display: flex;
  align-items: baseline;
  gap: 0.5em;
}

.p-inputtext, .p-button {
  align-self: center;
}

.tag-input {
  flex-grow: 1;
}

.p-inputswitch {
  align-self: center;
}
</style>
