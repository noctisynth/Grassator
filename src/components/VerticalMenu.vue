<script setup lang="ts">
defineProps(['menu']);
const activeMenu = defineModel<number>('activeMenu');
const disabled = defineModel<boolean>('disabled');
if (activeMenu.value === undefined) {
  activeMenu.value = 0;
}
if (disabled.value === undefined) {
  disabled.value = false;
}
</script>

<template>
  <ul class="shadow-lg bg-white dark:bg-p-surface-900">
    <li v-for="(item, index) in menu" :key="index">
      <div
        @click="activeMenu = index"
        :class="[
          'flex items-center gap-2 px-4 py-3 border-s-[3px] cursor-pointer',
          {
            'border-l-solid b-p-primary-500 bg-p-primary-100 text-p-primary-700':
              !disabled && index === activeMenu,
            'border-transparent text-gray-500 hover:border-gray-100 hover:bg-gray-50 hover:text-gray-700':
              disabled || index !== activeMenu,
          },
          {
            'dark:bg-p-primary-900 dark:text-blue-50':
              !disabled && index === activeMenu,
            'dark:text-gray-400 dark:hover:border-gray-700 dark:hover:bg-gray-800 dark:hover:text-gray-200':
              disabled || index !== activeMenu,
          },
        ]"
      >
        <i :class="[item.icon, 'opacity-75']"></i>

        <span class="text-sm font-medium"> {{ item.name }} </span>
      </div>
    </li>
  </ul>
</template>

<style scoped>
a {
  text-decoration: none;
}

ul {
  list-style-type: none;
  margin: unset;
  padding: unset;
}
</style>
