<script setup lang="ts">
import {ref, computed} from 'vue'
import InputText from 'primevue/inputtext'
import InputNumber from 'primevue/inputnumber'
import Button from 'primevue/button'
import InputSwitch from "primevue/inputswitch";

const props = defineProps<{
  editAllTags: boolean
}>()

const text = ref<string>('')
const position = ref<number>()

const emit = defineEmits<{
  (e: 'updateText', value: string[]): void
  (e: 'updatePosition', value: number | undefined): void
  (e: 'update:editAllTags', value: boolean): void
}>()

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
    <span>add tag</span>
    <input-text v-model="text" type="text"/>
    <span>position</span>
    <input-number v-model="position" placeholder="auto"
                  v-on:update:modelValue="emit('updatePosition', $event)"
                  :inputStyle="{ padding: '0.25em', width: '5em' }"/>
    <Button rounded class="p-button-x" v-on:click="emit('updateText', [text])">Insert</Button>
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
  padding: 0.25em;
  align-self: center;
}

.p-button {
  padding-left: 0.75em;
  padding-right: 0.75em;
}

.p-inputswitch {
  align-self: center;
}
</style>
