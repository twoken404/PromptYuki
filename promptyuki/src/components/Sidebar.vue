<template>
  <aside class="w-16 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col items-center py-4 gap-4 flex-shrink-0">
    <button
      @click="promptStore.createPrompt()"
      class="w-12 h-12 bg-blue-500 hover:bg-blue-600 text-white rounded-full flex items-center justify-center shadow-md transition-all active:scale-95"
      title="新建笔记"
    >
      <i class="i-lucide-plus text-xl"></i>
    </button>

    <div class="flex-1 flex flex-col gap-3 w-full items-center">
      <button
        @click="appStore.toggleTheme()"
        class="w-10 h-10 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center justify-center transition-colors"
        title="切换主题"
      >
        <i :class="appStore.isDark ? 'i-lucide-sun text-xl text-yellow-500' : 'i-lucide-moon text-xl text-gray-600 dark:text-gray-300'"></i>
      </button>
    </div>

    <div class="text-xs text-gray-400 dark:text-gray-500 font-medium">
      {{ promptStore.prompts.length }}
    </div>
  </aside>
</template>

<script setup lang="ts">
import { usePromptStore } from '../stores/prompt';
import { useAppStore } from '../stores/app';

const promptStore = usePromptStore();
const appStore = useAppStore();
</script>
