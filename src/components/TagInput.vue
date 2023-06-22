<script setup lang="ts">
import {ref, computed} from 'vue'
import InputText from 'primevue/inputtext'
import InputNumber from 'primevue/inputnumber'
import Button from 'primevue/button'
import InputSwitch from "primevue/inputswitch";

const props = defineProps<{
  editAllTags: boolean
}>()

const text = ref<string>()
const position = ref<string>()

const emit = defineEmits<{
  updateText: string[],
  updatePosition: number | undefined,
  'update:editAllTags': boolean
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
    <input-text v-model="text" type="text"></input-text>
    <span>position</span>
    <input-number v-model="position" placeholder="auto"
                  v-on:update:modelValue="emit('updatePosition', $event)"
                  inputStyle="padding: 0.25em; width: 5em"></input-number>
    <Button rounded class="p-button-x" v-on:click="emit('updateText', [text])">Insert</Button>
    <span>edit all tags</span>
    <input-switch v-model="editAllTags"></input-switch>
  </div>
</template>

<style scoped>
.tag-input-container {
  display: flex;
  align-items: baseline;
  padding: 0.5em;
  gap: 0.5em;
}

.p-inputtext, .p-button {
  padding: 0.25em;
}

.p-button {
  padding-left: 0.75em;
  padding-right: 0.75em;
}

.p-inputswitch {
  margin: 0;
  align-self: center;
}
</style>
