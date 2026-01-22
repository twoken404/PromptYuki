<template>
  <div class="flex-1 bg-gray-50 dark:bg-gray-900 flex flex-col w-full h-full overflow-hidden" @click="handleOutsideClick">
    <div class="max-w-5xl mx-auto w-full h-full flex flex-col">
      <!-- 新建按钮和搜索框 -->
      <div class="p-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 rounded-t-2xl flex-shrink-0">
        <div class="flex gap-3 items-center">
          <!-- 新建按钮 -->
          <button
            @click="promptStore.createPrompt()"
            class="w-12 h-12 bg-gradient-to-br from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white rounded-2xl flex items-center justify-center transition-all active:scale-95 flex-shrink-0"
            title="新建笔记"
          >
            <i class="i-lucide-plus text-2xl"></i>
          </button>

          <!-- 搜索框 -->
          <div class="relative flex-1">
            <i class="i-lucide-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"></i>
            <input
              v-model="promptStore.searchQuery"
              @keydown.esc="promptStore.searchQuery = ''"
              placeholder="搜索..."
              class="w-full pl-10 pr-10 py-3 bg-gray-50 dark:bg-gray-700 border border-gray-200/50 dark:border-gray-600/50 rounded-xl text-sm text-gray-800 dark:text-gray-200 placeholder-gray-400 focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 transition-all"
            />
            <button
              v-if="promptStore.searchQuery"
              @click="promptStore.searchQuery = ''"
              class="absolute right-3 top-1/2 -translate-y-1/2 w-5 h-5 flex items-center justify-center text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
            >
              <i class="i-lucide-x text-sm"></i>
            </button>
          </div>

          <!-- 设置 -->
          <button
            @click="showSettings = true"
            class="w-12 h-12 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-xl flex items-center justify-center transition-all flex-shrink-0"
            title="设置"
          >
            <i class="i-lucide-settings text-xl text-gray-600 dark:text-gray-300"></i>
          </button>

          <!-- 关于 -->
          <button
            @click="showAbout = true"
            class="w-12 h-12 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-xl flex items-center justify-center transition-all flex-shrink-0"
            title="关于"
          >
            <i class="i-lucide-info text-xl text-gray-600 dark:text-gray-300"></i>
          </button>

          <!-- 垃圾桶（回收站） -->
          <button
            @click="showTrash = true"
            class="w-12 h-12 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-xl flex items-center justify-center transition-all flex-shrink-0 relative"
            title="回收站"
          >
            <i class="i-lucide-trash-2 text-xl text-gray-600 dark:text-gray-300"></i>
            <span
              v-if="promptStore.trash.length > 0"
              class="absolute -top-1 -right-1 min-w-[20px] h-5 px-1 bg-green-500 text-white text-xs rounded-full flex items-center justify-center"
            >
              {{ promptStore.trash.length }}
            </span>
          </button>
        </div>
      </div>

      <!-- 笔记列表 -->
      <div class="flex-1 overflow-y-auto px-4 py-4 space-y-3 bg-white dark:bg-gray-800 rounded-b-2xl" @scroll="handleScroll">
        <div
          v-for="prompt in promptStore.filteredPrompts()"
          :key="prompt.id"
          class="relative group"
          @contextmenu.prevent="handleContextMenu($event, prompt.id)"
        >
          <!-- 卡片主体 -->
          <div
            @click="toggleEdit(prompt.id)"
            :class="[
              'p-4 rounded-xl cursor-pointer transition-all duration-200 border border-gray-100 dark:border-gray-700',
              promptStore.selectedPromptId === prompt.id
                ? 'ring-2 ring-blue-500 shadow-xl bg-gray-50 dark:bg-gray-900'
                : 'hover:shadow-lg shadow-sm hover:border-blue-200 dark:hover:border-blue-800'
            ]"
            @click.capture="handleCardClick"
          >
            <!-- 编辑模式的三个点菜单按钮 -->
            <div
              v-if="promptStore.selectedPromptId === prompt.id && promptStore.editing"
              class="absolute top-3 right-3"
              @click.stop="toggleMenu(prompt.id)"
            >
              <button
                data-menu-button="true"
                :class="[
                  'w-8 h-8 flex items-center justify-center rounded-lg transition-colors bg-gray-100 dark:bg-gray-700',
                  menuPromptId === prompt.id 
                    ? 'bg-gray-200 dark:bg-gray-600' 
                    : 'hover:bg-gray-200 dark:hover:bg-gray-600'
                ]"
              >
                <i class="i-lucide-more-horizontal text-gray-500 dark:text-gray-400"></i>
              </button>
            </div>

            <!-- 查看模式 -->
            <div v-if="!promptStore.editing || promptStore.selectedPromptId !== prompt.id">
              <h3 class="font-semibold text-sm text-gray-800 dark:text-gray-100 mb-2 truncate leading-snug">
                {{ prompt.title || '无标题' }}
              </h3>
              <p class="text-xs text-gray-500 dark:text-gray-400 line-clamp-3 leading-relaxed">
                {{ prompt.content || '无内容' }}
              </p>
              <div class="mt-3 pt-3 border-t border-gray-100 dark:border-gray-700 flex items-center justify-between">
                <span class="text-xs text-gray-400 dark:text-gray-500">
                  {{ formatTime(prompt.updatedAt) }}
                </span>
                <span class="text-xs text-gray-400 dark:text-gray-500">
                  {{ prompt.content?.length || 0 }} 字
                </span>
              </div>
            </div>

            <!-- 编辑模式 -->
            <div v-else class="space-y-3">
              <div class="space-y-1">
                <input
                  :value="prompt.title"
                  @input.stop="updateTitle(prompt.id, $event)"
                  @click.stop
                  placeholder="标题..."
                  class="w-full text-lg font-bold bg-transparent border-0 focus:outline-none text-gray-800 dark:text-white placeholder-gray-400"
                />
                <div class="h-px bg-gray-200 dark:bg-gray-600"></div>
              </div>
              <textarea
                ref="textareaRef"
                :value="prompt.content"
                @input.stop="updateContent(prompt.id, $event)"
                @click.stop
                placeholder="开始输入内容..."
                class="w-full bg-transparent text-sm text-gray-700 dark:text-gray-300 leading-relaxed placeholder-gray-400 focus:outline-none resize-none border-0 overflow-hidden"
                :style="{ height: getTextareaHeight(prompt.content || '') }"
              >              </textarea>
              <div class="flex items-center justify-start pt-3 border-t border-gray-200 dark:border-gray-600">
                <span class="text-xs text-gray-400 dark:text-gray-500">
                  {{ prompt.content?.length || 0 }} 字符
                </span>
              </div>
            </div>
          </div>

          <!-- 三个点下拉菜单 -->
          <div
            v-if="menuPromptId === prompt.id"
            class="menu-container absolute top-12 right-2 z-10 bg-white dark:bg-gray-800 rounded-lg border border-gray-200/50 dark:border-gray-700/50 py-0 min-w-[150px] overflow-hidden"
            @click.stop
          >
            <button
              @click="saveAsTxt(prompt.id)"
              class="w-full px-3 py-2 text-left text-sm text-gray-700 dark:text-gray-300 bg-transparent hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2 transition-colors border-b border-gray-200/30 dark:border-gray-700/30"
            >
              <i class="i-lucide-file-text text-base"></i>
              另存为TXT
            </button>
            <button
              @click="saveAsMd(prompt.id)"
              class="w-full px-3 py-2 text-left text-sm text-gray-700 dark:text-gray-300 bg-transparent hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2 transition-colors border-b border-gray-200/30 dark:border-gray-700/30"
            >
              <i class="i-lucide-file-code text-base"></i>
              另存为MD
            </button>
            <div class="h-px bg-gray-200/30 dark:bg-gray-700/30 mx-0 my-0"></div>
            <button
              @click="showAdvanced"
              class="w-full px-3 py-2 text-left text-sm text-gray-700 dark:text-gray-300 bg-transparent hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2 transition-colors border-b border-gray-200/30 dark:border-gray-700/30"
            >
              <i class="i-lucide-lock text-base"></i>
              高级功能
            </button>
            <button
              @click="moveToTrash(prompt.id)"
              class="w-full px-3 py-2 text-left text-sm text-red-500 bg-transparent hover:bg-red-50 dark:hover:bg-red-900/20 flex items-center gap-2 transition-colors border-b border-gray-200/30 dark:border-gray-700/30"
            >
              <i class="i-lucide-trash-2 text-base"></i>
              删除
            </button>
          </div>
        </div>

        <div
          v-if="promptStore.filteredPrompts().length === 0"
          class="text-center py-16 text-gray-400 dark:text-gray-500"
        >
          <i class="i-lucide-inbox text-5xl mb-3 mx-auto"></i>
          <p class="text-base font-medium">暂无笔记</p>
          <p class="text-sm mt-2 text-gray-400 dark:text-gray-500">点击上方 + 按钮新建</p>
        </div>
      </div>

      <!-- 底部状态栏 -->
      <div class="mt-3 px-4 py-3 border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 rounded-xl text-xs text-gray-400 dark:text-gray-500 flex items-center justify-between flex-shrink-0">
        <span>{{ promptStore.prompts.length }} 条笔记</span>
        <span class="flex items-center gap-1">
          <i class="i-lucide-database text-xs"></i>
          本地存储
        </span>
      </div>
    </div>

    <!-- 回收站弹窗 -->
    <div
      v-if="showTrash"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
      @click="handleOutsideClick"
    >
      <div
        class="bg-white dark:bg-gray-800 rounded-3xl shadow-2xl p-6 mx-4 max-w-md w-full max-h-[80vh] flex flex-col"
        @click.stop
      >
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-xl font-bold text-gray-800 dark:text-white">回收站</h2>
          <button
            @click="showTrash = false"
            class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            <i class="i-lucide-x text-gray-500 dark:text-gray-400"></i>
          </button>
        </div>

        <div class="flex-1 overflow-y-auto space-y-2">
          <div
            v-if="promptStore.trash.length === 0"
            class="text-center py-8 text-gray-400 dark:text-gray-500"
          >
            <i class="i-lucide-trash-2 text-4xl mb-2 mx-auto"></i>
            <p class="text-sm">回收站为空</p>
          </div>

          <div
            v-for="item in promptStore.trash"
            :key="item.originalId"
            class="p-3 rounded-xl border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-750 transition-colors"
          >
            <div class="flex items-start justify-between gap-3">
              <div class="flex-1 min-w-0">
                <h3 class="font-semibold text-sm text-gray-800 dark:text-gray-100 truncate">
                  {{ item.title || '无标题' }}
                </h3>
                <p class="text-xs text-gray-500 dark:text-gray-400 truncate mt-1">
                  {{ item.content || '无内容' }}
                </p>
                <p class="text-xs text-gray-400 dark:text-gray-500 mt-2">
                  {{ formatTime(item.deletedAt || item.updatedAt) }} 删除
                </p>
              </div>
              <div class="flex flex-col gap-1 flex-shrink-0">
                <button
                  @click="restoreItem(item.originalId)"
                  class="w-8 h-8 flex items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30 hover:bg-blue-200 dark:hover:bg-blue-900/50 transition-colors"
                  title="恢复"
                >
                  <i class="i-lucide-rotate-ccw text-sm text-blue-600 dark:text-blue-400"></i>
                </button>
                <button
                  @click="permanentlyDelete(item.originalId)"
                  class="w-8 h-8 flex items-center justify-center rounded-lg bg-red-100 dark:bg-red-900/30 hover:bg-red-200 dark:hover:bg-red-900/50 transition-colors"
                  title="永久删除"
                >
                  <i class="i-lucide-trash-2 text-sm text-red-600 dark:text-red-400"></i>
                </button>
              </div>
            </div>
          </div>
        </div>

        <div v-if="promptStore.trash.length > 0" class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
          <button
            @click="emptyTrash"
            class="w-full px-4 py-2.5 bg-red-500 hover:bg-red-600 text-white rounded-xl text-sm font-medium transition-colors"
          >
            清空回收站
          </button>
        </div>
      </div>
    </div>

    <!-- 复制成功提示 -->
    <div
      v-if="copyToast.show"
      class="fixed bottom-8 left-1/2 -translate-x-1/2 z-50 bg-gray-800 dark:bg-gray-200 text-white dark:text-gray-800 px-6 py-3 rounded-full shadow-lg transition-all duration-300"
      :class="copyToast.visible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-4'"
    >
      <div class="flex items-center gap-2">
        <i class="i-lucide-check text-green-400 dark:text-green-600"></i>
        <span class="text-sm">已复制</span>
      </div>
    </div>

    <!-- 关于弹窗 -->
    <div
      v-if="showAbout"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
      @click="showAbout = false"
    >
      <div
        class="bg-white dark:bg-gray-800 rounded-3xl shadow-2xl p-8 mx-4 max-w-md w-full"
        @click.stop
      >
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-800 dark:text-white">关于 PromptYuki</h2>
          <button
            @click="showAbout = false"
            class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            <i class="i-lucide-x text-gray-500 dark:text-gray-400"></i>
          </button>
        </div>

        <div class="space-y-4">
          <div class="flex items-center gap-3">
            <i class="i-lucide-sparkles text-2xl text-blue-500"></i>
            <div>
              <p class="font-semibold text-gray-800 dark:text-white">AI Prompt 管理工具</p>
              <p class="text-sm text-gray-500 dark:text-gray-400">版本 0.1.0</p>
            </div>
          </div>

          <div class="h-px bg-gray-200 dark:bg-gray-700"></div>

          <div>
            <h3 class="font-semibold text-gray-800 dark:text-white mb-2">快速开始</h3>
            <ul class="text-sm text-gray-600 dark:text-gray-400 space-y-2">
              <li class="flex items-start gap-2">
                <i class="i-lucide-plus text-green-500 mt-0.5"></i>
                <span>点击 + 按钮新建笔记</span>
              </li>
              <li class="flex items-start gap-2">
                <i class="i-lucide-edit-2 text-blue-500 mt-0.5"></i>
                <span>点击笔记进入编辑模式</span>
              </li>
              <li class="flex items-start gap-2">
                <i class="i-lucide-copy text-purple-500 mt-0.5"></i>
                <span>右键点击笔记复制内容</span>
              </li>
              <li class="flex items-start gap-2">
                <i class="i-lucide-download text-orange-500 mt-0.5"></i>
                <span>编辑时点击三个点导出文件</span>
              </li>
            </ul>
          </div>

          <div class="h-px bg-gray-200 dark:bg-gray-700"></div>

          <div class="text-center space-y-3">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">作者</p>
              <p class="font-semibold text-gray-800 dark:text-white">twoken</p>
            </div>
            <a
              href="https://github.com/twoken404/PromptYuki"
              target="_blank"
              class="inline-flex items-center gap-2 px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-xl text-sm text-gray-700 dark:text-gray-300 transition-colors"
            >
              <i class="i-lucide-github"></i>
              <span>GitHub 仓库</span>
            </a>
          </div>
        </div>
      </div>
    </div>

    <!-- 设置弹窗 -->
    <SettingsDialog v-model="showSettings" :onShortcutChange="registerGlobalShortcut" />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch, onMounted, onUnmounted } from 'vue';
import { usePromptStore } from '../stores/prompt';
import { useAppStore } from '../stores/app';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { appExit } from '@tauri-apps/plugin-process';
import { listen } from '@tauri-apps/api/event';
import SettingsDialog from './SettingsDialog.vue';

const promptStore = usePromptStore();
const appStore = useAppStore();

const showTrash = ref(false);
const showAbout = ref(false);
const showSettings = ref(false);
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const menuPromptId = ref<string | null>(null);

let unlistenShortcut: (() => void) | null = null;

  // 注册快捷键
  const registerGlobalShortcut = async (shortcut: string) => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('register_shortcut', { shortcut });
    } catch (e) {
      console.error('注册快捷键失败:', e);
    }
  };

  // 监听全局快捷键事件
  const setupShortcutListener = async () => {
    try {
      unlistenShortcut = await listen('global-shortcut-pressed', async () => {
        const window = getCurrentWebviewWindow();
        const isVisible = await window.isVisible();
        if (isVisible) {
          await window.hide();
        } else {
          await window.show();
          await window.setFocus();
        }
      });
    } catch (e) {
      console.error('监听快捷键失败:', e);
    }
  };

  // 同步最小化到托盘设置到 Rust 端
  const syncMinimizeToTray = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('set_minimize_to_tray', { minimize: appStore.minimizeToTray });
    } catch (e) {
      console.error('同步最小化设置失败:', e);
    }
  };

  onMounted(async () => {
    await setupShortcutListener();
    await registerGlobalShortcut(appStore.shortcutKey);
    await syncMinimizeToTray();

    // 监听 minimizeToTray 变化并同步到 Rust 端
    watch(() => appStore.minimizeToTray, async (newValue) => {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('set_minimize_to_tray', { minimize: newValue });
      } catch (e) {
        console.error('同步最小化设置失败:', e);
      }
    });

    // 监听 shortcutKey 变化并重新注册
    watch(() => appStore.shortcutKey, async (newShortcut) => {
      await registerGlobalShortcut(newShortcut);
    });
  });

onUnmounted(() => {
  if (unlistenShortcut) {
    unlistenShortcut();
  }
});

const copyToast = reactive({
  show: false,
  visible: false,
  timer: null as number | null
});

// 动态计算 textarea 高度
const getTextareaHeight = (content: string) => {
  const lines = content.split('\n');
  const lineHeight = 24; // 每行高度
  const padding = 20; // 内边距
  const minHeight = 120; // 最小高度
  const maxHeight = 600; // 最大高度

  // 计算需要的高度
  const calculatedHeight = Math.max(minHeight, lines.length * lineHeight + padding);
  return `${Math.min(calculatedHeight, maxHeight)}px`;
};

const showCopyToast = () => {
  copyToast.show = true;
  copyToast.visible = true;

  if (copyToast.timer) {
    clearTimeout(copyToast.timer);
  }

  copyToast.timer = window.setTimeout(() => {
    copyToast.visible = false;
    setTimeout(() => {
      copyToast.show = false;
    }, 300);
  }, 2000);
};

const toggleMenu = (id: string) => {
  if (menuPromptId.value === id) {
    menuPromptId.value = null;
  } else {
    menuPromptId.value = id;
  }
};

// 滚动时取消编辑状态但保留选中
const handleScroll = () => {
  if (promptStore.selectedPromptId && promptStore.editing) {
    promptStore.editing = false;
    menuPromptId.value = null;
  }
};

const handleContextMenu = async (e: MouseEvent, id: string) => {
  // 右键复制功能
  const prompt = promptStore.prompts.find(p => p.id === id);
  if (prompt) {
    await navigator.clipboard.writeText(prompt.content || '');
    showCopyToast();

    // 如果启用了复制后最小化到任务栏
    if (appStore.minimizeToTray) {
      await getCurrentWebviewWindow().hide();
    }
  }
};

const saveAsTxt = async (id: string) => {
  const prompt = promptStore.prompts.find(p => p.id === id);
  if (prompt) {
    const content = prompt.title + '\n\n' + prompt.content;
    try {
      const defaultPath = `${prompt.title || 'note'}.txt`;
      const filePath = await save({
        defaultPath,
        filters: [{
          name: 'Text File',
          extensions: ['txt']
        }]
      });
      if (filePath) {
        await writeTextFile(filePath, content);
      }
    } catch (error) {
      // 如果 Tauri 插件不可用，使用浏览器下载
      console.error('Tauri 保存失败，使用浏览器下载:', error);
      const blob = new Blob([content], { type: 'text/plain;charset=utf-8' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `${prompt.title || 'note'}.txt`;
      a.click();
      URL.revokeObjectURL(url);
    }
  }
  menuPromptId.value = null;
};

const saveAsMd = async (id: string) => {
  const prompt = promptStore.prompts.find(p => p.id === id);
  if (prompt) {
    const content = `# ${prompt.title}\n\n${prompt.content}`;
    try {
      const defaultPath = `${prompt.title || 'note'}.md`;
      const filePath = await save({
        defaultPath,
        filters: [{
          name: 'Markdown File',
          extensions: ['md', 'markdown']
        }]
      });
      if (filePath) {
        await writeTextFile(filePath, content);
      }
    } catch (error) {
      // 如果 Tauri 插件不可用，使用浏览器下载
      console.error('Tauri 保存失败，使用浏览器下载:', error);
      const blob = new Blob([content], { type: 'text/markdown;charset=utf-8' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `${prompt.title || 'note'}.md`;
      a.click();
      URL.revokeObjectURL(url);
    }
  }
  menuPromptId.value = null;
};

const showAdvanced = () => {
  // 高级功能预留
  alert('高级功能开发中...');
  menuPromptId.value = null;
};

const moveToTrash = (id: string) => {
  promptStore.moveToTrash(id);
  menuPromptId.value = null;
};

const handleLongPress = (id: string) => {
  // App 端长按复制
  const prompt = promptStore.prompts.find(p => p.id === id);
  if (prompt) {
    navigator.clipboard.writeText(prompt.content || '');
    showCopyToast();
  }
};

const restoreItem = (originalId: string) => {
  promptStore.restoreFromTrash(originalId);
};

const permanentlyDelete = (originalId: string) => {
  promptStore.trash = promptStore.trash.filter(t => t.originalId !== originalId);
};

const emptyTrash = () => {
  promptStore.emptyTrash();
};

const toggleEdit = (id: string) => {
  if (promptStore.selectedPromptId === id && promptStore.editing) {
    promptStore.editing = false;
    promptStore.selectedPromptId = null;
    menuPromptId.value = null;
  } else {
    promptStore.selectPrompt(id);
    promptStore.editing = true;
    menuPromptId.value = null;
  }
};

const finishEdit = (id: string) => {
  promptStore.editing = false;
  promptStore.selectedPromptId = null;
  menuPromptId.value = null;
};

const updateTitle = (id: string, e: Event) => {
  const target = e.target as HTMLInputElement;
  promptStore.updatePrompt(id, { title: target.value });
};

const updateContent = (id: string, e: Event) => {
  const target = e.target as HTMLTextAreaElement;
  promptStore.updatePrompt(id, { content: target.value });
};

const formatTime = (timestamp: number) => {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 1) return '刚刚';
  if (minutes < 60) return `${minutes}分钟前`;
  if (hours < 24) return `${hours}小时前`;
  if (days < 7) return `${days}天前`;
  return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' });
};

const handleCardClick = (e: MouseEvent) => {
  // 如果当前笔记正在编辑且菜单打开，点击卡片其他区域关闭菜单
  const prompt = promptStore.prompts.find(p => p.id === promptStore.selectedPromptId);
  if (prompt && promptStore.editing && menuPromptId.value === promptStore.selectedPromptId) {
    // 检查点击是否在菜单按钮或菜单内
    const target = e.target as HTMLElement;
    const isMenuButton = target.closest('[data-menu-button="true"]');
    const isMenu = target.closest('.menu-container');
    
    if (!isMenuButton && !isMenu) {
      menuPromptId.value = null;
      e.stopPropagation();
    }
  }
};
</script>

<style>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

textarea {
  min-height: 120px;
  resize: vertical;
}
</style>
