<script setup lang="ts">
import { onMounted } from 'vue';
import { loadConfig } from './scripts/config';

onMounted(async () => {
  loadConfig();
});
</script>

<template>
  <RouterView />
</template>

<style scoped></style>
