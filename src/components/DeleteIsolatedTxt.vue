<script setup lang="ts">
import {ref, watch} from 'vue'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import {showDialog, deleteList, deleteDir} from './DeleteIsolatedTxt'
import Checkbox from 'primevue/checkbox'

import {removeFile} from '@tauri-apps/api/fs'
import {join} from '@tauri-apps/api/path'

const confirmList = ref<{ file: string, select: boolean }[]>([])

watch(deleteList, value => {
  confirmList.value = value.map(x => ({file: x, select: true}))
})

async function confirmDelete() {
  for (const node of confirmList.value) {
    if (node.select)
      await removeFile(await join(deleteDir, node.file))
  }
  showDialog.value = false
}
</script>

<template>
  <Dialog v-model:visible="showDialog" modal :style="{ width: '50rem' }">
    <template #header>
      <div>Select the files you want to clean</div>
    </template>
    <div v-if="confirmList.length">
      <div v-for="node in confirmList" class="confirm-item">
        <checkbox v-model="node.select" binary :input-id="`checkbox-${node.file}`"/>
        <label :for="`checkbox-${node.file}`">{{ node.file }}</label>
      </div>
    </div>
    <p v-else>
      There are no files to clean up.
    </p>
    <template #footer>
      <Button v-if="confirmList.length" label="Ok" icon="pi pi-check" v-on:click="confirmDelete"/>
      <Button label="Cancel" v-on:click="showDialog = false" autofocus/>
    </template>
  </Dialog>
</template>

<style scoped>
.confirm-item {
  margin: 8px;
  display: flex;
  gap: 4px;
}

.confirm-item > checkbox {
  font-size: 0.5rem;
}
</style>