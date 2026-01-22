<script setup lang="ts">
import { onMounted } from 'vue';
import { useAppStore } from './stores/app';
import { usePromptStore } from './stores/prompt';
import PromptList from './components/PromptList.vue';

const appStore = useAppStore();
const promptStore = usePromptStore();

onMounted(() => {
  appStore.initTheme();
});
</script>

<template>
  <div class="h-screen w-screen overflow-hidden bg-gray-50 dark:bg-gray-900 flex">
    <PromptList />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Microsoft YaHei', sans-serif;
  -webkit-font-smoothing: antialiased;
  overscroll-behavior: none;
}

textarea {
  resize: none;
  outline: none;
}

/* 隐藏滚动条但保留滚动功能 */
textarea::-webkit-scrollbar,
div::-webkit-scrollbar {
  width: 0px;
  display: none;
}

/* Firefox */
textarea,
div {
  scrollbar-width: none;
  -ms-overflow-style: none;
}

textarea::-webkit-scrollbar-thumb,
div::-webkit-scrollbar-thumb {
  background-color: #d1d5db;
  border-radius: 3px;
}

.dark textarea::-webkit-scrollbar-thumb,
.dark div::-webkit-scrollbar-thumb {
  background-color: #4b5563;
}
</style>
