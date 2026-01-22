import { defineStore } from 'pinia';
import { ref, onMounted, watch } from 'vue';

export const useAppStore = defineStore('app', () => {
  const isDark = ref(false);
  const isMobile = ref(false);
  const minimizeToTray = ref(true);
  const shortcutKey = ref('Control+Backquote');

  // 初始化主题
  const initTheme = () => {
    const saved = localStorage.getItem('promptyuki_theme');
    if (saved) {
      isDark.value = saved === 'dark';
    } else {
      isDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    applyTheme();
  };

  const applyTheme = () => {
    if (isDark.value) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  };

  const toggleTheme = () => {
    isDark.value = !isDark.value;
    localStorage.setItem('promptyuki_theme', isDark.value ? 'dark' : 'light');
    applyTheme();
  };

  // 检测是否为移动端
  const checkMobile = () => {
    isMobile.value = window.innerWidth < 768;
  };

  // 从 localStorage 加载设置
  const loadSettings = () => {
    const savedMinimize = localStorage.getItem('promptyuki_minimize_to_tray');
    if (savedMinimize !== null) {
      minimizeToTray.value = savedMinimize === 'true';
    }

    const savedShortcut = localStorage.getItem('promptyuki_shortcut_key');
    if (savedShortcut) {
      shortcutKey.value = savedShortcut;
    }
  };

  // 保存设置到 localStorage
  const saveSettings = () => {
    localStorage.setItem('promptyuki_minimize_to_tray', minimizeToTray.value.toString());
    localStorage.setItem('promptyuki_shortcut_key', shortcutKey.value);
  };

  // 监听设置变化并自动保存
  watch([minimizeToTray, shortcutKey], () => {
    saveSettings();
  });

  onMounted(() => {
    checkMobile();
    window.addEventListener('resize', checkMobile);
    loadSettings();
  });

  return {
    isDark,
    isMobile,
    minimizeToTray,
    shortcutKey,
    initTheme,
    toggleTheme,
    saveSettings,
  };
});
