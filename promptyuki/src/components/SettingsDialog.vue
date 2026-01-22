<template>
  <div
    v-if="show"
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    @click="handleOutsideClick"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-3xl shadow-2xl p-8 mx-4 max-w-md w-full"
      @click.stop
    >
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-2xl font-bold text-gray-800 dark:text-white">设置</h2>
        <button
          @click="show = false"
          class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
        >
          <i class="i-lucide-x text-gray-500 dark:text-gray-400"></i>
        </button>
      </div>

      <div class="space-y-6">
        <!-- 快捷键设置 -->
        <div>
          <label class="block text-sm font-semibold text-gray-800 dark:text-white mb-3">
            全局快捷键
          </label>
          <div
            tabindex="0"
            @keydown="handleShortcutInput"
            class="w-full px-4 py-2.5 bg-gray-50 dark:bg-gray-700 border border-gray-200/50 dark:border-gray-600/50 rounded-xl text-sm text-gray-800 dark:text-gray-200 focus:outline-none focus:border-blue-500/50 transition-colors cursor-pointer"
            :class="{ 'ring-2 ring-blue-500': isRecording }"
          >
            <span v-if="isRecording" class="text-gray-400">请按下快捷键...</span>
            <span v-else>{{ appStore.shortcutKey || '点击设置快捷键' }}</span>
          </div>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
            点击上方输入框，然后按下要使用的快捷键
          </p>
        </div>

        <!-- 最小化到任务栏 -->
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-semibold text-gray-800 dark:text-white">最小化到任务栏</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
              关闭窗口时最小化到托盘而不是退出
            </p>
          </div>
          <button
            @click="toggleMinimizeToTray"
            :class="[
              'relative w-12 h-6 rounded-full transition-colors duration-200',
              appStore.minimizeToTray ? 'bg-blue-500' : 'bg-gray-300 dark:bg-gray-600'
            ]"
          >
            <span
              :class="[
                'absolute top-1 w-4 h-4 bg-white rounded-full transition-transform duration-200 shadow',
                appStore.minimizeToTray ? 'left-7' : 'left-1'
              ]"
            ></span>
          </button>
        </div>

        <div class="h-px bg-gray-200 dark:bg-gray-700"></div>

        <!-- 主题切换 -->
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-semibold text-gray-800 dark:text-white">主题</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
              切换明暗主题
            </p>
          </div>
          <button
            @click="appStore.toggleTheme()"
            class="w-12 h-12 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-xl flex items-center justify-center transition-all"
          >
            <i :class="appStore.isDark ? 'i-lucide-sun text-xl text-yellow-500' : 'i-lucide-moon text-xl text-gray-600 dark:text-gray-300'"></i>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useAppStore } from '../stores/app';

const props = defineProps<{
  modelValue: boolean;
  onShortcutChange?: (shortcut: string) => void;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
}>();

const appStore = useAppStore();
const show = ref(props.modelValue);
const isRecording = ref(false);

const handleOutsideClick = () => {
  show.value = false;
  emit('update:modelValue', false);
};

const handleShortcutInput = async (e: KeyboardEvent) => {
  e.preventDefault();
  isRecording.value = true;

  // 阻止默认行为
  e.stopPropagation();

  // 获取按键组合
  const keys: string[] = [];
  if (e.ctrlKey) keys.push('Ctrl');
  if (e.altKey) keys.push('Alt');
  if (e.shiftKey) keys.push('Shift');
  if (e.metaKey) keys.push('Meta');

  // 获取主键（排除修饰键）
  const mainKey = e.code;
  if (
    !['ControlLeft', 'ControlRight', 'AltLeft', 'AltRight', 'ShiftLeft', 'ShiftRight', 'MetaLeft', 'MetaRight'].includes(mainKey)
  ) {
    // 映射按键名称
    const keyMap: Record<string, string> = {
      'KeyA': 'A', 'KeyB': 'B', 'KeyC': 'C', 'KeyD': 'D', 'KeyE': 'E',
      'KeyF': 'F', 'KeyG': 'G', 'KeyH': 'H', 'KeyI': 'I', 'KeyJ': 'J',
      'KeyK': 'K', 'KeyL': 'L', 'KeyM': 'M', 'KeyN': 'N', 'KeyO': 'O',
      'KeyP': 'P', 'KeyQ': 'Q', 'KeyR': 'R', 'KeyS': 'S', 'KeyT': 'T',
      'KeyU': 'U', 'KeyV': 'V', 'KeyW': 'W', 'KeyX': 'X', 'KeyY': 'Y', 'KeyZ': 'Z',
      'Digit1': '1', 'Digit2': '2', 'Digit3': '3', 'Digit4': '4', 'Digit5': '5',
      'Digit6': '6', 'Digit7': '7', 'Digit8': '8', 'Digit9': '9', 'Digit0': '0',
      'Space': 'Space', 'Enter': 'Enter', 'Escape': 'Esc',
      'Backspace': 'Backspace', 'Tab': 'Tab',
      'F1': 'F1', 'F2': 'F2', 'F3': 'F3', 'F4': 'F4', 'F5': 'F5',
      'F6': 'F6', 'F7': 'F7', 'F8': 'F8', 'F9': 'F9', 'F10': 'F10',
      'F11': 'F11', 'F12': 'F12'
    };

    const mappedKey = keyMap[mainKey] || mainKey;
    if (mappedKey) {
      keys.push(mappedKey);
    }
  }

  // 如果只有修饰键，不设置
  if (keys.length > 0 && keys.some(k => !['Ctrl', 'Alt', 'Shift', 'Meta'].includes(k))) {
    const shortcut = keys.join('+');
    appStore.shortcutKey = shortcut;

    // 注册新快捷键
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('register_shortcut', { shortcut });

      // 通知父组件快捷键已更改
      if (props.onShortcutChange) {
        props.onShortcutChange(shortcut);
      }
    } catch (e) {
      console.error('注册快捷键失败:', e);
    }
  }

  isRecording.value = false;
};

  const toggleMinimizeToTray = async () => {
    appStore.minimizeToTray = !appStore.minimizeToTray;
  };

// 监听外部变化
watch(() => props.modelValue, (newVal) => {
  show.value = newVal;
});

// 监听内部变化
watch(show, (newVal) => {
  emit('update:modelValue', newVal);
});
</script>
