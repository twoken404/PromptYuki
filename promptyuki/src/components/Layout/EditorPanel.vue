<template>
  <div class="flex-1 bg-white dark:bg-dark-bg flex flex-col h-full">
    <div v-if="promptStore.selectedPrompt" class="h-full flex flex-col">
      <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
        <input
          v-model="editingPrompt.title"
          class="text-xl font-bold bg-transparent border-0 focus:outline-none text-gray-800 dark:text-white flex-1"
          placeholder="Prompt 标题"
        />
        <div class="flex items-center gap-2">
          <button
            @click="savePrompt"
            class="px-4 py-1.5 bg-primary hover:bg-primary-dark text-white rounded-lg text-sm font-medium transition-colors"
          >
            保存
          </button>
          <button
            @click="copyToClipboard"
            class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
            title="复制到剪贴板"
          >
            <i class="i-lucide-copy text-gray-600 dark:text-gray-400"></i>
          </button>
          <button
            @click="deleteCurrentPrompt"
            class="p-2 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
            title="删除"
          >
            <i class="i-lucide-trash-2 text-gray-600 dark:text-gray-400 hover:text-red-500"></i>
          </button>
        </div>
      </div>

      <div class="flex-1 flex overflow-hidden">
        <div class="flex-1 p-0">
          <MarkdownEditor
            v-model="editingPrompt.content"
            :variables="editingPrompt.variables"
          />
        </div>

        <div class="w-80 border-l border-gray-200 dark:border-gray-700 p-4 overflow-y-auto">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-white mb-3">标签</h3>
          <div class="flex flex-wrap gap-2 mb-4">
            <span
              v-for="(tag, index) in editingPrompt.tags"
              :key="index"
              class="inline-flex items-center gap-1 px-2 py-1 bg-primary/10 text-primary rounded-full text-xs"
            >
              {{ tag }}
              <button @click="removeTag(index)" class="hover:text-primary-dark">
                <i class="i-lucide-x text-xs"></i>
              </button>
            </span>
            <input
              v-model="newTag"
              @keyup.enter="addTag"
              placeholder="添加标签..."
              class="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-800 border-0 rounded focus:outline-none focus:ring-1 focus:ring-primary text-gray-700 dark:text-gray-300 w-20"
            />
          </div>

          <h3 class="text-sm font-semibold text-gray-800 dark:text-white mb-3">元数据</h3>
          <div class="space-y-3 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-500 dark:text-gray-400">使用次数</span>
              <span class="text-gray-800 dark:text-white">{{ promptStore.selectedPrompt.usageCount }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500 dark:text-gray-400">创建时间</span>
              <span class="text-gray-800 dark:text-white">{{ formatDate(promptStore.selectedPrompt.metadata.createdAt) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500 dark:text-gray-400">最后使用</span>
              <span class="text-gray-800 dark:text-white">
                {{ promptStore.selectedPrompt.metadata.lastUsedAt
                  ? formatDate(promptStore.selectedPrompt.metadata.lastUsedAt)
                  : '未使用' }}
              </span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500 dark:text-gray-400">字数</span>
              <span class="text-gray-800 dark:text-white">{{ promptStore.selectedPrompt.metadata.wordCount }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="h-full flex items-center justify-center">
      <div class="text-center">
        <i class="i-lucide-file-text text-6xl text-gray-300 dark:text-gray-600 mb-4"></i>
        <p class="text-gray-500 dark:text-gray-400">选择一个 Prompt 开始编辑</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-2">或创建一个新 Prompt</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { usePromptStore } from '../../stores/prompt';
import MarkdownEditor from '../Editor/MarkdownEditor.vue';
import type { Prompt } from '../../types';

const promptStore = usePromptStore();

const editingPrompt = ref<Prompt>({
  id: '',
  title: '',
  content: '',
  description: '',
  variables: [],
  tags: [],
  folderId: '',
  isFavorite: false,
  usageCount: 0,
  metadata: {
    createdAt: new Date(),
    updatedAt: new Date(),
    wordCount: 0,
  },
});

const newTag = ref('');

watch(() => promptStore.selectedPrompt, (newPrompt) => {
  if (newPrompt) {
    editingPrompt.value = JSON.parse(JSON.stringify(newPrompt));
  } else {
    editingPrompt.value = {
      id: '',
      title: '',
      content: '',
      description: '',
      variables: [],
      tags: [],
      folderId: '',
      isFavorite: false,
      usageCount: 0,
      metadata: {
        createdAt: new Date(),
        updatedAt: new Date(),
        wordCount: 0,
      },
    };
  }
}, { deep: true });

const savePrompt = async () => {
  if (!editingPrompt.value.title || !editingPrompt.value.content) {
    alert('请填写标题和内容');
    return;
  }

  editingPrompt.value.metadata.wordCount = editingPrompt.value.content.split(/\s+/).filter(w => w).length;
  await promptStore.savePrompt(editingPrompt.value);
  alert('保存成功');
};

const deleteCurrentPrompt = async () => {
  if (promptStore.selectedPromptId && confirm('确定要删除这个 Prompt 吗？')) {
    await promptStore.deletePrompt(promptStore.selectedPromptId);
  }
};

const copyToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(editingPrompt.value.content);
    promptStore.incrementUsage(editingPrompt.value.id);
    alert('已复制到剪贴板');
  } catch (err) {
    console.error('复制失败:', err);
  }
};

const addTag = () => {
  if (newTag.value.trim() && !editingPrompt.value.tags.includes(newTag.value.trim())) {
    editingPrompt.value.tags.push(newTag.value.trim());
    newTag.value = '';
  }
};

const removeTag = (index: number) => {
  editingPrompt.value.tags.splice(index, 1);
};

const formatDate = (date: Date) => {
  return new Date(date).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  });
};
</script>
