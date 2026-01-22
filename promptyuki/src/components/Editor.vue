<template>
  <div class="flex-1 flex flex-col relative border-l border-gray-200 dark:border-gray-700">
    <div v-if="promptStore.selectedPromptId" class="h-full flex flex-col">
      <!-- 移动端返回按钮 -->
      <button
        v-if="appStore.isMobile"
        @click="promptStore.selectPrompt(null)"
        class="md:hidden p-4 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2 text-gray-600 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-750"
      >
        <i class="i-lucide-arrow-left"></i>
        <span>返回</span>
      </button>

      <!-- 标题栏 -->
      <div class="px-6 py-5 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between bg-gradient-to-r from-white to-gray-50 dark:from-gray-800 dark:to-gray-850 sticky top-0 z-10 shadow-sm">
        <input
          :value="selectedPrompt?.title"
          @input="updateTitle"
          placeholder="输入标题..."
          class="text-2xl font-bold bg-transparent border-0 focus:outline-none text-gray-800 dark:text-white flex-1 placeholder-gray-400"
        />
        <button
          @click="deleteCurrent"
          class="ml-4 px-5 py-2.5 bg-red-500 hover:bg-red-600 active:bg-red-700 text-white rounded-xl transition-all duration-200 font-medium text-sm shadow-md hover:shadow-lg flex items-center gap-2"
          title="删除笔记"
        >
          <i class="i-lucide-trash-2 text-base"></i>
          <span>删除</span>
        </button>
      </div>

      <!-- 编辑区 -->
      <div class="flex-1 relative bg-white dark:bg-gray-800">
        <textarea
          :value="selectedPrompt?.content"
          @input="updateContent"
          placeholder="开始输入你的内容..."
          class="w-full h-full p-8 bg-transparent text-gray-700 dark:text-gray-300 text-base leading-7 placeholder-gray-400 focus:outline-none"
        ></textarea>

        <!-- 背景提示文字 -->
        <div v-if="!selectedPrompt?.content" class="absolute inset-0 flex items-center justify-center pointer-events-none bg-gradient-to-br from-white to-gray-50 dark:from-gray-800 dark:to-gray-850">
          <div class="text-center">
            <div class="w-16 h-16 bg-blue-100 dark:bg-blue-900/20 rounded-2xl flex items-center justify-center mx-auto mb-4">
              <i class="i-lucide-file-text text-3xl text-blue-500 dark:text-blue-400"></i>
            </div>
            <p class="text-gray-400 dark:text-gray-500 text-base mb-1">在这里记录你的 Prompt</p>
            <p class="text-gray-300 dark:text-gray-600 text-sm">所有内容自动保存</p>
          </div>
        </div>
      </div>

      <!-- 状态栏 -->
      <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 bg-gradient-to-r from-gray-50 to-white dark:from-gray-850 dark:to-gray-800 text-sm text-gray-500 dark:text-gray-400 flex items-center justify-between shadow-inner">
        <div class="flex items-center gap-4">
          <span class="flex items-center gap-1.5">
            <i class="i-lucide-type text-xs"></i>
            {{ selectedPrompt?.content?.length || 0 }} 字符
          </span>
          <span class="text-gray-300 dark:text-gray-600">|</span>
          <span class="flex items-center gap-1.5">
            <i class="i-lucide-clock text-xs"></i>
            {{ formatTime(selectedPrompt?.updatedAt || Date.now()) }}
          </span>
        </div>
        <span class="flex items-center gap-1.5 text-green-500 dark:text-green-400">
          <i class="i-lucide-check-circle text-xs"></i>
          已自动保存
        </span>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="h-full flex items-center justify-center bg-gradient-to-br from-gray-50 via-white to-gray-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900">
      <div class="text-center max-w-md px-6">
        <div class="w-24 h-24 bg-gradient-to-br from-blue-100 to-blue-200 dark:from-blue-900/30 dark:to-blue-800/20 rounded-3xl flex items-center justify-center mx-auto mb-6 shadow-lg">
          <i class="i-lucide-pen-tool text-5xl text-blue-500 dark:text-blue-400"></i>
        </div>
        <h2 class="text-3xl font-bold text-gray-800 dark:text-white mb-3">欢迎使用 PromptYuki</h2>
        <p class="text-gray-500 dark:text-gray-400 mb-8 text-base leading-relaxed">
          极致简洁的 AI Prompt 管理工具<br>
          点击左侧 <span class="text-blue-500 font-semibold">+</span> 按钮开始记录
        </p>
        <button
          @click="promptStore.createPrompt()"
          class="px-10 py-4 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 active:scale-[0.98] text-white rounded-2xl transition-all duration-200 font-semibold text-base shadow-xl hover:shadow-2xl flex items-center gap-2 mx-auto"
        >
          <i class="i-lucide-plus text-lg"></i>
          创建第一个笔记
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { usePromptStore } from '../stores/prompt';
import { useAppStore } from '../stores/app';

const promptStore = usePromptStore();
const appStore = useAppStore();

const selectedPrompt = computed(() =>
  promptStore.prompts.find(p => p.id === promptStore.selectedPromptId)
);

const updateTitle = (e: Event) => {
  const target = e.target as HTMLInputElement;
  promptStore.updatePrompt(promptStore.selectedPromptId!, { title: target.value });
};

const updateContent = (e: Event) => {
  const target = e.target as HTMLTextAreaElement;
  promptStore.updatePrompt(promptStore.selectedPromptId!, { content: target.value });
};

const deleteCurrent = () => {
  if (promptStore.selectedPromptId && confirm('确定要删除这条笔记吗？')) {
    promptStore.deletePrompt(promptStore.selectedPromptId);
  }
};

const formatTime = (timestamp: number) => {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 1) return '刚刚';
  if (minutes < 60) return `${minutes} 分钟前`;
  if (hours < 24) return `${hours} 小时前`;
  if (days < 7) return `${days} 天前`;
  return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' });
};
</script>

<style>
textarea {
  font-size: 16px;
  line-height: 1.75;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.text-center > div {
  animation: fadeIn 0.3s ease-out;
}
</style>
