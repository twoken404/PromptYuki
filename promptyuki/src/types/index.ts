export interface Prompt {
  id: string;
  title: string;
  content: string;
  description?: string;
  variables: Variable[];
  tags: string[];
  folderId?: string;
  isFavorite: boolean;
  usageCount: number;
  rating?: number;
  metadata: {
    createdAt: Date;
    updatedAt: Date;
    lastUsedAt?: Date;
    wordCount: number;
  };
}

export interface Variable {
  name: string;
  type: 'text' | 'select' | 'multi-select' | 'file';
  defaultValue?: any;
  options?: string[];
  required: boolean;
  placeholder?: string;
}

export interface Folder {
  id: string;
  name: string;
  icon?: string;
  color?: string;
  parentId?: string;
  sortOrder: number;
}

export interface Tag {
  id: string;
  name: string;
  color: string;
  count: number;
}

export type ViewMode = 'edit' | 'preview';
