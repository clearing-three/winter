import react from '@eslint-react/eslint-plugin';
import prettierConfig from 'eslint-config-prettier';
import importX from 'eslint-plugin-import-x';
import jsonc from 'eslint-plugin-jsonc';
import jsxA11y from 'eslint-plugin-jsx-a11y';
import promise from 'eslint-plugin-promise';
import reactRefresh from 'eslint-plugin-react-refresh';
import security from 'eslint-plugin-security';
import sonarjs from 'eslint-plugin-sonarjs';
import toml from 'eslint-plugin-toml';
import unicorn from 'eslint-plugin-unicorn';
import yml from 'eslint-plugin-yml';
import tseslint from 'typescript-eslint';

export default [
  {
    ignores: ['dist', 'node_modules', 'pnpm-lock.yaml'],
  },
  {
    files: ['**/*.{js,mjs,cjs,ts,tsx}'],
    ...tseslint.configs.recommended[0],
    ...importX.flatConfigs.recommended,
    ...importX.flatConfigs.typescript,
    ...unicorn.configs.recommended,
    ...promise.configs['flat/recommended'],
    ...sonarjs.configs.recommended,
    ...security.configs.recommended,
  },
  // Type-aware TypeScript rules for source files
  ...tseslint.configs.strictTypeChecked.map((config) => ({
    ...config,
    files: ['src/**/*.{ts,tsx}'],
  })),
  ...tseslint.configs.stylisticTypeChecked.map((config) => ({
    ...config,
    files: ['src/**/*.{ts,tsx}'],
  })),
  {
    files: ['src/**/*.{ts,tsx}'],
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    files: ['**/*.tsx'],
    ...react.configs.recommended,
    ...jsxA11y.flatConfigs.recommended,
    plugins: {
      'react-refresh': reactRefresh,
    },
    rules: {
      'react-refresh/only-export-components': 'error',
    },
  },
  ...jsonc.configs['flat/recommended-with-jsonc'],
  {
    files: ['**/*.json', '**/*.jsonc'],
    rules: {
      'jsonc/sort-keys': 'error',
    },
  },
  ...yml.configs['flat/recommended'],
  ...yml.configs['flat/prettier'],
  {
    files: ['**/*.{yml,yaml}'],
    rules: {
      'yml/sort-keys': 'error',
    },
  },
  ...toml.configs['flat/recommended'],
  {
    files: ['**/*.toml'],
    rules: {
      'toml/keys-order': 'error',
    },
  },
  prettierConfig,
];
