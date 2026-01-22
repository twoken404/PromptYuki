import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface SimplePrompt {
  id: string;
  title: string;
  content: string;
  createdAt: number;
  updatedAt: number;
  deleted?: boolean;
  deletedAt?: number;
}

export interface TrashItem extends SimplePrompt {
  originalId: string;
}

export const usePromptStore = defineStore('prompt', () => {
  const prompts = ref<SimplePrompt[]>([]);
  const trash = ref<TrashItem[]>([]);
  const selectedPromptId = ref<string | null>(null);
  const searchQuery = ref('');
  const editing = ref(false);

  // 从 localStorage 加载数据
  const loadFromStorage = () => {
    const saved = localStorage.getItem('promptyuki_prompts');
    const savedTrash = localStorage.getItem('promptyuki_trash');
    if (saved) {
      try {
        prompts.value = JSON.parse(saved);
      } catch (e) {
        console.error('Failed to load prompts:', e);
      }
    }
    if (savedTrash) {
      try {
        trash.value = JSON.parse(savedTrash);
      } catch (e) {
        console.error('Failed to load trash:', e);
      }
    }
  };

  // 保存到 localStorage
  const saveToStorage = () => {
    localStorage.setItem('promptyuki_prompts', JSON.stringify(prompts.value));
    localStorage.setItem('promptyuki_trash', JSON.stringify(trash.value));
  };

  // 初始化时加载数据
  loadFromStorage();

  const filteredPrompts = () => {
    const activePrompts = prompts.value.filter(p => !p.deleted);
    if (!searchQuery.value) {
      // 按 updatedAt 降序排序
      return [...activePrompts].sort((a, b) => b.updatedAt - a.updatedAt);
    }
    const query = searchQuery.value.toLowerCase();
    return activePrompts
      .filter(p =>
        p.title.toLowerCase().includes(query) ||
        p.content.toLowerCase().includes(query)
      )
      .sort((a, b) => b.updatedAt - a.updatedAt);
  };

  const selectPrompt = (id: string | null) => {
    selectedPromptId.value = id;
  };

  const createPrompt = () => {
    const newPrompt: SimplePrompt = {
      id: Date.now().toString(),
      title: '新笔记',
      content: '',
      createdAt: Date.now(),
      updatedAt: Date.now(),
    };
    prompts.value.unshift(newPrompt);
    saveToStorage();
    selectPrompt(newPrompt.id);
    editing.value = true;
    return newPrompt;
  };

  const updatePrompt = (id: string, updates: Partial<SimplePrompt>) => {
    const index = prompts.value.findIndex(p => p.id === id);
    if (index > -1) {
      prompts.value[index] = {
        ...prompts.value[index],
        ...updates,
        updatedAt: Date.now(),
      };
      saveToStorage();
    }
  };

  const moveToTrash = (id: string) => {
    const index = prompts.value.findIndex(p => p.id === id);
    if (index > -1) {
      const prompt = prompts.value[index];
      prompts.value.splice(index, 1);
      trash.value.unshift({
        ...prompt,
        originalId: prompt.id,
        deletedAt: Date.now(),
      } as TrashItem);
      saveToStorage();
      if (selectedPromptId.value === id) {
        selectedPromptId.value = null;
        editing.value = false;
      }
    }
  };

  const restoreFromTrash = (originalId: string) => {
    const trashIndex = trash.value.findIndex(t => t.originalId === originalId);
    if (trashIndex > -1) {
      const item = trash.value[trashIndex];
      trash.value.splice(trashIndex, 1);
      prompts.value.unshift({
        ...item,
        id: item.originalId,
      });
      saveToStorage();
    }
  };

  const deletePrompt = (id: string) => {
    const index = prompts.value.findIndex(p => p.id === id);
    if (index > -1) {
      prompts.value.splice(index, 1);
      saveToStorage();
      if (selectedPromptId.value === id) {
        selectedPromptId.value = null;
        editing.value = false;
      }
    }
  };

  const emptyTrash = () => {
    trash.value = [];
    saveToStorage();
  };

  return {
    prompts,
    trash,
    selectedPromptId,
    searchQuery,
    editing,
    filteredPrompts,
    selectPrompt,
    createPrompt,
    updatePrompt,
    deletePrompt,
    moveToTrash,
    restoreFromTrash,
    emptyTrash,
  };
});
