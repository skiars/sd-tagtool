<script setup lang="ts">
import {RouterView} from 'vue-router'
</script>

<template>
  <router-view v-slot="{ Component }">
    <keep-alive include="Home">
      <component :is="Component"/>
    </keep-alive>
  </router-view>
</template>

<style scoped>

</style>