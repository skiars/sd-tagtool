<script setup lang="ts">
import {ref, shallowRef} from 'vue'
import MenuBar from 'primevue/menubar'
import {Menu} from "../lib/types";

const emit = defineEmits<{
  action: Menu
}>()

function action(a: Menu) {
  return () => emit('action', a)
}

const items = ref([{
  label: 'File',
  icon: 'pi pi-fw pi-file',
  items: [{
    label: 'Open',
    icon: 'pi pi-fw pi-plus',
    command: action(Menu.Open)
  },{
    label: 'Save',
    icon: 'pi pi-fw pi-save',
    command: action(Menu.Save)
  }, {
    label: 'Quit',
    icon: 'pi pi-fw pi-power-off',
    command: action(Menu.Quit)
  }]
}, {
  label: 'Edit',
  icon: 'pi pi-fw pi-pencil',
  items: [
    {
      label: 'Undo',
      icon: 'pi pi-fw pi-align-left',
      command: action(Menu.Undo)
    },
    {
      label: 'Redo',
      icon: 'pi pi-fw pi-align-right',
      command: action(Menu.Redo)
    }
  ]
}]);
</script>

<template>
  <MenuBar :model="items">
  </MenuBar>
</template>

<style scoped>
.p-menubar {
  padding: 0.25rem;
}
</style>