<script setup lang="ts">
// import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref, shallowReactive } from 'vue';
// import { FileSize } from '../types';
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
const searchText = ref<string>('');

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
  // const size: FileSize = await invoke('get_file_size', {
  //   url: 'https://objects.githubusercontent.com/github-production-release-asset-2e65be/27574418/6423efd7-c4f4-424b-8e1e-c8d1f88148d2?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=releaseassetproduction%2F20240716%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240716T084950Z&X-Amz-Expires=300&X-Amz-Signature=1ec3fc4d425eb93890634c3cbf6d2774a411ba17e5bf80aa9cac3a149120ed3c&X-Amz-SignedHeaders=host&actor_id=46275354&key_id=0&repo_id=27574418&response-content-disposition=attachment%3B%20filename%3DCascadiaCode.zip&response-content-type=application%2Foctet-stream',
  // });
  // console.log(size.error ? size.error : `File size: ${size.size}`);
  // await invoke('download_file', {
  //   url: 'https://objects.githubusercontent.com/github-production-release-asset-2e65be/27574418/6423efd7-c4f4-424b-8e1e-c8d1f88148d2?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=releaseassetproduction%2F20240716%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240716T084950Z&X-Amz-Expires=300&X-Amz-Signature=1ec3fc4d425eb93890634c3cbf6d2774a411ba17e5bf80aa9cac3a149120ed3c&X-Amz-SignedHeaders=host&actor_id=46275354&key_id=0&repo_id=27574418&response-content-disposition=attachment%3B%20filename%3DCascadiaCode.zip&response-content-type=application%2Foctet-stream',
  //   output: 'VSCodeUserSetup-x64-1.91.1.exe',
  // });
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
        <div class="flex items-center gap-2">
          <Avatar image="/logo.png"></Avatar>
          <IconField v-tooltip="'Unimplemented now'">
            <InputIcon>
              <i class="pi pi-search"></i>
            </InputIcon>
            <InputText
              v-model="searchText"
              size="small"
              placeholder="Search"
              disabled
            />
          </IconField>
        </div>

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

      <div class="flex flex-1 h-full w-full">
        <VerticalMenu
          class="fixed w-32 h-full transform max-sm:-translate-x-full transition-transform duration-300"
          :menu="menu"
          :activeMenu="activeMenu"
        />
      </div>
    </div>
  </main>
</template>

<style scoped></style>
