import stylistic from '@stylistic/eslint-plugin';
import simpleImportSort from 'eslint-plugin-simple-import-sort';
import tailwind from 'eslint-plugin-tailwindcss';
import pluginVue from 'eslint-plugin-vue';
import tseslint from 'typescript-eslint';
import vueParser from 'vue-eslint-parser';

export default [
  {
    ignores: ['dist/**', 'node_modules/**', 'src/api/generated/**/*'],
  },
  ...tseslint.configs.recommended,
  ...pluginVue.configs['flat/recommended'],

  {
    files: ['**/*.js', '**/*.ts', '**/*.vue'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslint.parser,
        extraFileExtensions: ['.vue'],
        sourceType: 'module',
      },
    },
    plugins: {
      '@stylistic': stylistic,
      'simple-import-sort': simpleImportSort,
    },
    rules: {
      'curly': 'error',
      'eqeqeq': ['error', 'always', { 'null': 'ignore' }],
      'no-console': ['warn', { allow: ['warn', 'error'] }],
      'simple-import-sort/exports': 'error',
      'simple-import-sort/imports': 'error',

      '@stylistic/array-bracket-spacing': ['error', 'never'],
      '@stylistic/arrow-spacing': ['error', { 'before': true, 'after': true }],
      '@stylistic/block-spacing': ['error', 'always'],
      '@stylistic/comma-dangle': ['error', 'always-multiline'],
      '@stylistic/comma-spacing': 'error',
      '@stylistic/eol-last': 'error',
      '@stylistic/indent': ['error', 2],
      '@stylistic/keyword-spacing': ['error', { 'before': true, 'after': true }],
      '@stylistic/max-len': ['warn', { code: 130, tabWidth: 2, ignoreUrls: true, ignoreStrings: true }],
      '@stylistic/member-delimiter-style': 'error',
      '@stylistic/no-multi-spaces': 'error',
      '@stylistic/no-multiple-empty-lines': ['error', { 'max': 1, 'maxEOF': 1 }],
      '@stylistic/no-trailing-spaces': 'error',
      '@stylistic/object-curly-spacing': ['error', 'always'],
      '@stylistic/quotes': ['error', 'single', { avoidEscape: true }],
      '@stylistic/semi': ['error', 'always'],
      '@stylistic/space-before-blocks': ['error', 'always'],
      '@stylistic/space-in-parens': ['error', 'never'],
      '@stylistic/space-infix-ops': 'error',
      '@stylistic/template-curly-spacing': ['error', 'never'],
    },
  },

  {
    files: ['**/*.vue'],
    plugins: {
      'tailwind': tailwind,
    },
    rules: {
      'vue/array-bracket-spacing': ['error', 'never'],
      'vue/attributes-order': ['error', { alphabetical: true }],
      'vue/block-lang': ['error', { script: { lang: 'ts' } }],
      'vue/block-order': ['error', { order: ['template', 'script', 'style'] }],
      'vue/define-macros-order': 'error',
      'vue/html-indent': ['error', 2],
      'vue/max-attributes-per-line': ['error', { singleline: { max: 6 }, multiline: { max: 1 } }],
      'vue/mustache-interpolation-spacing': ['error', 'always'],
      'vue/object-curly-spacing': ['error', 'always'],
      'vue/prefer-true-attribute-shorthand': ['error', 'always'],
      'vue/space-in-parens': ['error', 'never'],
      'vue/space-infix-ops': 'error',
      'vue/template-curly-spacing': ['error', 'never'],

      'tailwind/classnames-order': 'error',
      'tailwind/enforces-negative-arbitrary-values': 'error',
      'tailwind/enforces-shorthand': 'error',
      'tailwind/no-custom-classname': 'warn',
      'tailwind/no-unnecessary-arbitrary-value': 'error',
    },
  },

  {
    files: ['src/pages/**/*.vue'],
    rules: {
      'vue/multi-word-component-names': 'off',
    },
  },
];
