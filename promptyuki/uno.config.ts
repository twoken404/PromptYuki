import { defineConfig, presetAttributify, presetUno, presetIcons } from 'unocss'

export default defineConfig({
  presets: [
    presetAttributify(),
    presetUno(),
    presetIcons({
      scale: 1.2,
      warn: true,
      collections: {
        lucide: () => import('@iconify-json/lucide/icons.json').then(i => i.default),
      },
    }),
  ],
  theme: {
    colors: {
      primary: '#4F8AFF',
      'primary-dark': '#3A6FD9',
      snow: '#F8FAFC',
      'dark-bg': '#1E293B',
      'dark-surface': '#334155',
    },
    borderRadius: {
      sm: '4px',
      DEFAULT: '8px',
      lg: '12px',
    },
  },
})
