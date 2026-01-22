<template>
  <aside class="w-60 bg-snow dark:bg-dark-bg border-r border-gray-200 dark:border-gray-700 flex flex-col h-full transition-all duration-300">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <h1 class="text-xl font-bold text-gray-800 dark:text-white flex items-center gap-2">
        <i class="i-lucide-snowflake text-primary"></i>
        PromptYuki
      </h1>
    </div>

    <nav class="flex-1 overflow-y-auto p-3">
      <div class="space-y-1">
        <button
          v-for="folder in promptStore.folders"
          :key="folder.id"
          @click="selectFolder(folder.id)"
          :class="[
            'w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors',
            promptStore.selectedFolderId === folder.id
              ? 'bg-primary text-white'
              : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800'
          ]"
        >
          <i v-if="folder.icon" :class="[folder.icon, 'text-lg']"></i>
          <span class="text-sm font-medium">{{ folder.name }}</span>
        </button>
      </div>

      <div class="mt-6">
        <h3 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2 px-3">
          标签
        </h3>
        <div class="space-y-1">
          <button
            v-for="tag in promptStore.tags"
            :key="tag.id"
            @click="promptStore.toggleTagFilter(tag.name)"
            :class="[
              'w-full flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-colors',
              promptStore.selectedTagFilter.includes(tag.name)
                ? 'bg-gray-100 dark:bg-gray-800 text-primary'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800/50'
            ]"
          >
            <span class="w-2 h-2 rounded-full" :style="{ backgroundColor: tag.color }"></span>
            <span>{{ tag.name }}</span>
            <span class="ml-auto text-xs text-gray-400">{{ tag.count }}</span>
          </button>
        </div>
      </div>
    </nav>

    <div class="p-3 border-t border-gray-200 dark:border-gray-700">
      <button
        @click="appStore.toggleTheme"
        class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
      >
        <i :class="appStore.isDark ? 'i-lucide-sun' : 'i-lucide-moon'"></i>
        <span class="text-sm">{{ appStore.isDark ? '亮色模式' : '暗色模式' }}</span>
      </button>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { usePromptStore } from '../../stores/prompt';
import { useAppStore } from '../../stores/app';

const promptStore = usePromptStore();
const appStore = useAppStore();

const selectFolder = (id: string) => {
  promptStore.selectFolder(id);
};
</script>
