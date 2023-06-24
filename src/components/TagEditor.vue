<script setup lang="ts">
import {ref, computed, watch} from 'vue'
import {invoke} from '@tauri-apps/api/tauri'
import InputText from 'primevue/inputtext'
import InputNumber from 'primevue/inputnumber'
import InputSwitch from 'primevue/inputswitch'

const props = defineProps<{
  editAllTags: boolean
}>()

const emit = defineEmits<{
  (e: 'updateTags', value: string[]): void
  (e: 'updatePosition', value: number | undefined): void
  (e: 'update:editAllTags', value: boolean): void
}>()

const text = ref<string>('')
const tags = ref<string[]>([])
const position = ref<number>()

watch(text, async x =>
    tags.value = await invoke('parse_tags', {text: x})
)

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
    <input-text v-model="text" type="text" v-on:keydown.enter="emit('updateTags', tags)"
                placeholder="Separate tags with ',' and press Enter to insert" />
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

.p-inputtext {
  flex-grow: 1;
}

.p-button {
  padding-left: 0.75em;
  padding-right: 0.75em;
}

.p-inputswitch {
  align-self: center;
}
</style>
