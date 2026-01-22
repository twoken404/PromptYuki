<template>
  <div class="h-full flex flex-col">
    <div class="flex items-center justify-between px-4 py-2 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-dark-surface">
      <div class="flex items-center gap-2">
        <button
          @click="toggleView('edit')"
          :class="[
            'px-3 py-1.5 rounded text-sm font-medium transition-colors',
            viewMode === 'edit'
              ? 'bg-white dark:bg-dark-surface text-primary shadow-sm'
              : 'text-gray-600 dark:text-gray-400 hover:bg-white dark:hover:bg-dark-surface'
          ]"
        >
          <i class="i-lucide-edit-3 mr-1"></i>
          编辑
        </button>
        <button
          @click="toggleView('preview')"
          :class="[
            'px-3 py-1.5 rounded text-sm font-medium transition-colors',
            viewMode === 'preview'
              ? 'bg-white dark:bg-dark-surface text-primary shadow-sm'
              : 'text-gray-600 dark:text-gray-400 hover:bg-white dark:hover:bg-dark-surface'
          ]"
        >
          <i class="i-lucide-eye mr-1"></i>
          预览
        </button>
        <button
          @click="toggleView('split')"
          :class="[
            'px-3 py-1.5 rounded text-sm font-medium transition-colors',
            viewMode === 'split'
              ? 'bg-white dark:bg-dark-surface text-primary shadow-sm'
              : 'text-gray-600 dark:text-gray-400 hover:bg-white dark:hover:bg-dark-surface'
          ]"
        >
          <i class="i-lucide-columns mr-1"></i>
          分屏
        </button>
      </div>
      <div v-if="variables.length > 0" class="flex items-center gap-2">
        <span class="text-xs text-gray-500 dark:text-gray-400">变量:</span>
        <input
          v-for="variable in variables"
          :key="variable.name"
          :placeholder="variable.placeholder || variable.name"
          class="px-2 py-1 text-sm bg-white dark:bg-dark-bg border border-gray-200 dark:border-gray-600 rounded focus:outline-none focus:ring-1 focus:ring-primary"
          @input="updateVariable(variable.name, $event)"
        />
      </div>
    </div>

    <div class="flex-1 flex overflow-hidden" :class="{ 'flex-row': viewMode === 'split' }">
      <div v-show="viewMode === 'edit' || viewMode === 'split'" class="flex-1 p-4 overflow-y-auto">
        <textarea
          :value="modelValue"
          @input="$emit('update:modelValue', $event.target.value)"
          class="w-full h-full resize-none border-0 focus:outline-none text-gray-700 dark:text-gray-300 bg-transparent font-mono text-sm leading-relaxed"
          placeholder="输入 Prompt 内容... 支持 Markdown 语法"
        ></textarea>
      </div>

      <div v-show="viewMode === 'preview' || viewMode === 'split'" class="flex-1 p-4 overflow-y-auto border-l border-gray-200 dark:border-gray-700">
        <div class="markdown-preview prose dark:prose-invert max-w-none" v-html="renderedMarkdown"></div>
      </div>
    </div>

    <div class="px-4 py-2 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-dark-surface text-xs text-gray-500 dark:text-gray-400 flex justify-between">
      <span>{{ modelValue.length }} 字符</span>
      <span>{{ modelValue.split(/\s+/).filter(w => w).length }} 单词</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { marked } from 'marked';
import type { Variable } from '../../types';

interface Props {
  modelValue: string;
  variables?: Variable[];
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  variables: () => [],
});

const emit = defineEmits<Emits>();

const viewMode = ref<'edit' | 'preview' | 'split'>('edit');
const variableValues = ref<Record<string, string>>({});

const toggleView = (mode: 'edit' | 'preview' | 'split') => {
  viewMode.value = mode;
};

const renderedMarkdown = computed(() => {
  let content = props.modelValue;

  // Replace variable placeholders with their values
  props.variables.forEach(variable => {
    const value = variableValues.value[variable.name] || variable.placeholder || `{{${variable.name}}}`;
    const regex = new RegExp(`\\{\\{${variable.name}\\}\\}`, 'g');
    content = content.replace(regex, value);
  });

  return marked(content);
});

const updateVariable = (name: string, event: Event) => {
  const target = event.target as HTMLInputElement;
  variableValues.value[name] = target.value;
};
</script>

<style>
.markdown-preview {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  font-size: 14px;
  line-height: 1.6;
  color: #374151;
}

.dark .markdown-preview {
  color: #e5e7eb;
}

.markdown-preview h1,
.markdown-preview h2,
.markdown-preview h3,
.markdown-preview h4,
.markdown-preview h5,
.markdown-preview h6 {
  font-weight: 600;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
  line-height: 1.25;
}

.markdown-preview h1 {
  font-size: 2em;
  border-bottom: 1px solid #e5e7eb;
  padding-bottom: 0.3em;
}

.dark .markdown-preview h1 {
  border-bottom-color: #374151;
}

.markdown-preview h2 {
  font-size: 1.5em;
  border-bottom: 1px solid #e5e7eb;
  padding-bottom: 0.3em;
}

.dark .markdown-preview h2 {
  border-bottom-color: #374151;
}

.markdown-preview code {
  background-color: #f3f4f6;
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-size: 0.875em;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}

.dark .markdown-preview code {
  background-color: #374151;
}

.markdown-preview pre {
  background-color: #1f2937;
  padding: 1em;
  border-radius: 6px;
  overflow-x: auto;
  margin: 1em 0;
}

.markdown-preview pre code {
  background-color: transparent;
  padding: 0;
  color: #e5e7eb;
}

.markdown-preview blockquote {
  border-left: 4px solid #4F8AFF;
  padding-left: 1em;
  margin: 1em 0;
  color: #6b7280;
}

.markdown-preview ul,
.markdown-preview ol {
  padding-left: 2em;
  margin: 1em 0;
}

.markdown-preview li {
  margin: 0.25em 0;
}

.markdown-preview a {
  color: #4F8AFF;
  text-decoration: none;
}

.markdown-preview a:hover {
  text-decoration: underline;
}
</style>
