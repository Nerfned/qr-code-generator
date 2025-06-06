const ruleError = process.env.NODE_ENV === 'production' ? 'error' : 'warn';

module.exports = {
  root: true,
  env: {
    node: true,
  },
  extends: [
    'plugin:vue/vue3-essential',
    'eslint:recommended',
    '@vue/typescript',
  ],
  parserOptions: {
    parser: '@typescript-eslint/parser',
  },
  rules: {
    'comma-dangle': [ruleError, 'always-multiline'],
    'comma-style': [ruleError, 'last'],
    'no-empty': ruleError,
    'no-console': ruleError,
    'no-debugger': ruleError,
    'no-useless-escape': ruleError,
    'object-shorthand': ruleError,
    'prefer-const': ruleError,
    'prefer-destructuring': ruleError,
    'prefer-template': ruleError,
    'quotes': [ruleError, 'single', { 'avoidEscape': true, 'allowTemplateLiterals': true }],
    'semi': ruleError,
    'sort-imports': ruleError,
    'vue/attribute-hyphenation': 'off',
    'vue/html-indent': [ruleError, 4],
    'vue/html-self-closing': [ruleError, {
      'html': {
        'void': 'always',
        'normal': 'always',
        'component': 'always',
      },
      'svg': 'always',
      'math': 'always',
    }],
    'vue/max-attributes-per-line': [ruleError, {
      'singleline': {
        'max': 1,
      },
      'multiline': {
        'max': 1,
      },
    }],
    'vue/multi-word-component-names': 'off',
    'vue/no-unused-components': ruleError,
    'vue/no-v-html': 'off',
    'vue/no-v-model-argument': 'off',
    'vue/prop-name-casing': 'off',

    '@typescript-eslint/consistent-type-imports': ruleError,

    // bei gelegenheit anpassen
    'vue/no-multiple-template-root': 'off',
  },
};
