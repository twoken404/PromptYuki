<template>
  <div class="w-96 bg-white dark:bg-dark-surface border-r border-gray-200 dark:border-gray-700 flex flex-col h-full">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <div class="relative">
        <i class="i-lucide-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"></i>
        <input
          v-model="promptStore.searchQuery"
          placeholder="搜索 Prompt..."
          class="w-full pl-10 pr-4 py-2 bg-gray-100 dark:bg-gray-800 border-0 rounded-lg text-sm text-gray-800 dark:text-gray-200 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-primary"
        />
      </div>
    </div>

    <div class="flex-1 overflow-y-auto p-3 space-y-2">
      <div
        v-for="prompt in promptStore.filteredPrompts"
        :key="prompt.id"
        @click="promptStore.selectPrompt(prompt.id)"
        :class="[
          'p-3 rounded-lg cursor-pointer transition-all',
          promptStore.selectedPromptId === prompt.id
            ? 'bg-primary text-white shadow-lg'
            : 'bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700'
        ]"
      >
        <div class="flex items-start gap-2">
          <div class="flex-1 min-w-0">
            <h3 class="font-medium text-sm truncate mb-1">
              {{ prompt.title }}
            </h3>
            <p class="text-xs line-clamp-2 mb-2" :class="promptStore.selectedPromptId === prompt.id ? 'text-white/80' : 'text-gray-500 dark:text-gray-400'">
              {{ prompt.description || prompt.content.substring(0, 100) }}
            </p>
            <div class="flex items-center gap-2 flex-wrap">
              <span
                v-for="tag in prompt.tags"
                :key="tag"
                class="text-xs px-2 py-0.5 rounded-full"
                :class="promptStore.selectedPromptId === prompt.id ? 'bg-white/20' : 'bg-gray-200 dark:bg-gray-700'"
              >
                {{ tag }}
              </span>
            </div>
          </div>
          <button
            @click.stop="promptStore.toggleFavorite(prompt.id)"
            class="p-1 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
          >
            <i
              :class="[
                prompt.isFavorite ? 'i-lucide-star' : 'i-lucide-star',
                promptStore.selectedPromptId === prompt.id ? 'text-yellow-300' : prompt.isFavorite ? 'text-yellow-500' : 'text-gray-400 hover:text-yellow-500'
              ]"
            ></i>
          </button>
        </div>
      </div>

      <div
        v-if="promptStore.filteredPrompts.length === 0"
        class="text-center py-8 text-gray-400 dark:text-gray-500"
      >
        <i class="i-lucide-inbox text-4xl mb-2 mx-auto"></i>
        <p class="text-sm">没有找到 Prompt</p>
      </div>
    </div>

    <div class="p-3 border-t border-gray-200 dark:border-gray-700">
      <button
        @click="createNewPrompt"
        class="w-full flex items-center justify-center gap-2 px-4 py-2 bg-primary hover:bg-primary-dark text-white rounded-lg transition-colors text-sm font-medium"
      >
        <i class="i-lucide-plus"></i>
        新建 Prompt
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { usePromptStore } from '../../stores/prompt';

const promptStore = usePromptStore();

const createNewPrompt = () => {
  // TODO: Implement create new prompt
  console.log('Create new prompt');
};
</script>
