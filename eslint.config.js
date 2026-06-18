import js from '@eslint/js'
import tsParser from '@typescript-eslint/parser'
import tsPlugin from '@typescript-eslint/eslint-plugin'
import svelte from 'eslint-plugin-svelte'
import globals from 'globals'

export default [
  {
    ignores: ['.svelte-kit/**', 'build/**', 'node_modules/**', 'src-tauri/**']
  },
  js.configs.recommended,
  ...svelte.configs.recommended,
  {
    rules: {
      'no-unused-vars': 'off'
    }
  },
  {
    files: ['src/**/*.ts'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: 'module'
      },
      globals: {
        ...globals.browser,
        ...globals.node,
        $derived: 'readonly',
        $effect: 'readonly',
        $props: 'readonly',
        $state: 'readonly'
      }
    },
    plugins: {
      '@typescript-eslint': tsPlugin
    },
    rules: {
      ...tsPlugin.configs.recommended.rules,
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }]
    }
  },
  {
    files: ['src/**/*.svelte'],
    languageOptions: {
      parserOptions: {
        parser: tsParser
      },
      globals: globals.browser
    }
  }
]
