<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted } from 'vue';
import { FileSize } from '../types';

onMounted(async () => {
    const size: FileSize = await invoke('get_file_size', { url: 'https://freetestdata.com/wp-content/uploads/2021/09/1-MB-DOC.doc' })
    console.log(size.error ? size.error : `File size: ${size.size}`);
})
</script>

<template>
    <main class="h-full w-full">
        <div class="flex h-full w-full justify-center items-center">
            <h1 class="text-4xl font-bold">Welcome to Grassator</h1>
        </div>
    </main>
</template>

<style scoped></style>