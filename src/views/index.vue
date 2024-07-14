<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref, shallowReactive } from 'vue';
import { FileSize } from '../types';
import { config } from '../scripts/config';

const menu = shallowReactive([
  {
    icon: 'pi pi-home',
    name: 'Home',
    link: '/',
  },
  {
    icon: 'pi pi-cog',
    name: 'Settings',
    link: '/settings',
  },
]);
const activeMenu = ref<number>(0);

const theme = ref(
  new Proxy(
    {
      mode: 'light',
    },
    {
      get: (target, prop) => Reflect.get(target, prop),
      set: (target, _, value) => {
        if (value === 'light') {
          document.documentElement.classList.contains('dark') &&
            document.documentElement.classList.remove('dark');
        } else {
          document.documentElement.classList.contains('dark') ||
            document.documentElement.classList.add('dark');
        }
        return Reflect.set(target, 'mode', value);
      },
    }
  )
);

onMounted(async () => {
  const size: FileSize = await invoke('get_file_size', {
    url: 'https://freetestdata.com/wp-content/uploads/2021/09/1-MB-DOC.doc',
  });
  console.log(size.error ? size.error : `File size: ${size.size}`);
  await invoke('download_file', {
    url: 'https://freetestdata.com/wp-content/uploads/2021/09/1-MB-DOC.doc',
    output: '1-MB-DOC.doc',
  });
  // We synchronize the config object with the Rust backend
  config.num_threads = 12;
  console.log(config);
  config.num_threads = 39;
  console.log(config);
});
</script>

<template>
  <main class="h-full w-full">
    <Toast class="max-w-90%"></Toast>
    <div class="flex flex-col h-full w-full">
      <div
        :class="[
          'flex justify-between items-center h-12 px-4',
          'bg-p-surface-0 b-solid b-[1px] b-p-surface-200',
          'dark:bg-p-surface-900 dark:b-p-surface-700',
        ]"
      >
        <IconField>
          <InputIcon>
            <i class="pi pi-search"></i>
          </InputIcon>
          <InputText placeholder="Search" disabled />
        </IconField>

        <div>
          <Button
            @click="theme.mode = theme.mode === 'light' ? 'dark' : 'light'"
            :icon="theme.mode === 'light' ? 'pi pi-sun' : 'pi pi-moon'"
            severity="secondary"
            text
          ></Button>
          <Button
            icon="pi pi-sort-alt"
            class="ml-2"
            severity="secondary"
            text
            disabled
          ></Button>
          <Button
            icon="pi pi-plus"
            class="ml-2"
            severity="secondary"
            text
            disabled
          ></Button>
        </div>
      </div>

      <VerticalMenu
        class="w-64 h-full hidden sm:block"
        :menu="menu"
        :activeMenu="activeMenu"
        disabled
      />
    </div>
  </main>
</template>

<style scoped></style>
