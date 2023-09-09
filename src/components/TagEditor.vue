<script setup lang="ts">
import {ref, computed} from 'vue'
import Button from 'primevue/button'
import InputNumber from 'primevue/inputnumber'
import InputSwitch from 'primevue/inputswitch'
import TagInput from './TagInput.vue'

const props = defineProps<{
  editAllTags: boolean,
}>()

const emit = defineEmits<{
  (e: 'updateTags', value: string[]): void
  (e: 'updatePosition', value: number | undefined): void
  (e: 'update:editAllTags', value: boolean): void,
  (e: 'replaceTags', value: {from: string[], to: string[]}): void
}>()

const position = ref<number>()
const tags = ref<string[]>([])
const replaceTags = ref<string[]>([])
const more = ref<boolean>()

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
  <div class="outer-container">
    <div class="tag-input-container">
      <i :class="['pi', 'pi-angle-right', more ? 'rotate-90' : undefined]"
         v-on:click="more = !more"/>
      <tag-input v-model="tags" placeholder="Separate tags with ',' or press Enter"/>
      <span>position</span>
      <input-number v-model="position" placeholder="auto"
                    v-on:update:modelValue="emit('updatePosition', $event)"
                    :inputStyle="{ padding: '0.25em', width: '4em' }"/>
      <Button class="fixed" rounded v-on:click="emit('updateTags', tags)">
        Insert
      </Button>
      <span>edit all tags</span>
      <input-switch class="fixed" v-model="editAllTags"/>
    </div>
    <div v-if="more" class="tag-input-container">
      <span>replace with</span>
      <tag-input v-model="replaceTags" placeholder="Separate tags with ',' or press Enter"/>
      <Button class="fixed" rounded
              v-on:click="emit('replaceTags', {from: tags, to: replaceTags})">
        Replace
      </Button>
    </div>
  </div>
</template>

<style scoped>
.outer-container {
  display: flex;
  flex-direction: column;
  gap: 0.5em;
}

.tag-input-container {
  display: flex;
  align-items: baseline;
  gap: 0.5em;
}

.fixed {
  flex-shrink: 0;
  align-self: flex-start;
}

.p-inputtext, .p-button {
  align-self: center;
}

.p-inputswitch {
  align-self: center;
}


.fixed {
  flex-shrink: 0;
  align-self: flex-start;
}

i {
  font-size: 1.1em;
  border: 1.5px solid #888;
  border-radius: 1em;
  transition: transform 0.3s;
}

.rotate-90 {
  transform: rotate(90deg);
}
</style>
