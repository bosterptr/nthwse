module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: [
    'airbnb',
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:import/typescript',
    'plugin:prettier/recommended',
    'plugin:react-hooks/recommended',
    'plugin:react/recommended',
    'plugin:storybook/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 12,
    sourceType: 'module',
    project: './tsconfig.json',
    tsconfigRootDir: './',
  },
  plugins: ['react', '@typescript-eslint', 'import', 'react-hooks', 'formatjs', 'prettier'],
  rules: {
    '@typescript-eslint/explicit-module-boundary-types': ['off'],
    '@typescript-eslint/no-use-before-define': ['error'],
    'formatjs/enforce-default-message': ['error', 'literal'],
    'formatjs/enforce-placeholders': ['error', { ignoreList: ['b'] }],
    'formatjs/no-id': ['error'],
    'formatjs/no-offset': ['error'],
    'import/extensions': 'off',
    'import/no-extraneous-dependencies': ['error', { devDependencies: true }],
    'import/no-unresolved': 'off',
    'import/prefer-default-export': 'off',
    'no-param-reassign': ['error', { props: true, ignorePropertyModificationsFor: ['draft'] }],
    'no-restricted-imports': 'off',
    'no-use-before-define': 'off',
    'prettier/prettier': ['error'],
    'react/display-name': [0, { ignoreTranspilerName: false }],
    'react/function-component-definition': [
      0,
      {
        namedComponents: 'function-declaration',
        unnamedComponents: 'arrow-function',
      },
    ],
    'react/jsx-filename-extension': [1, { extensions: ['.tsx'] }],
    'react/prop-types': 'off',
    'no-restricted-syntax': [
      // Nicer booleans #1
      // Disabling the use of !! to cast to boolean
      'error',
      {
        selector: 'UnaryExpression[operator="!"] > UnaryExpression[operator="!"]',
        message: '!! to cast to boolean relies on a double negative. Use Boolean() instead',
      },
      // Nicer booleans #2
      // Avoiding accidental `new Boolean()` calls
      // (also covered by `no-new-wrappers` but i am having fun)
      {
        selector: 'NewExpression[callee.name="Boolean"]',
        message:
          'Avoid using constructor: `new Boolean(value)` as it creates a Boolean object. Did you mean `Boolean(value)`?',
      },
      // We are using a useLayoutEffect / useEffect switch to avoid SSR warnings for useLayoutEffect
      // We want to ensure we use `import useEffect from '*use-isomorphic-layout-effect'`
      // to ensure we still get the benefits of `eslint-plugin-react-hooks`
      {
        selector:
          'ImportDeclaration[source.value=/use-isomorphic-layout-effect/] > ImportDefaultSpecifier[local.name!="useLayoutEffect"]',
        message:
          'Must use `useLayoutEffect` as the name of the import from `*use-isomorphic-layout-effect` to leverage `eslint-plugin-react-hooks`',
      },

      // No Array.from as it pulls in a large amount of babel helpers
      {
        selector: 'MemberExpression[object.name="Array"][property.name="from"]',
        message:
          'Not allowing using of Array.from to save kbs. Please use native-with-fallback/from',
      },
    ],

    // Not requiring default prop declarations all the time
    'react/require-default-props': 'off',

    '@typescript-eslint/adjacent-overload-signatures': 'error',
    '@typescript-eslint/consistent-type-definitions': ['error', 'interface'],
    '@typescript-eslint/consistent-type-assertions': 'error',
    '@typescript-eslint/member-ordering': [
      'error',
      {
        default: [
          'static-field',
          'static-method',
          'instance-field',
          'constructor',
          'abstract-method',
          'instance-method',
        ],
      },
    ],
    '@typescript-eslint/naming-convention': [
      'error',
      {
        selector: 'variable',
        format: ['camelCase', 'UPPER_CASE', 'PascalCase'],
        leadingUnderscore: 'allowSingleOrDouble',
      },
      {
        selector: 'variable',
        modifiers: ['destructured'],
        format: null,
      },
      {
        selector: 'typeLike',
        format: ['PascalCase'],
      },
      {
        selector: 'enum',
        format: ['PascalCase', 'UPPER_CASE'],
      },
    ],
    '@typescript-eslint/no-explicit-any': 'error',
    '@typescript-eslint/no-namespace': ['error', { allowDeclarations: true }],
    '@typescript-eslint/no-invalid-this': 'off',
    '@typescript-eslint/no-shadow': 'error',
    '@typescript-eslint/no-var-requires': 'error',
    '@typescript-eslint/prefer-for-of': 'error',
    '@typescript-eslint/prefer-namespace-keyword': 'error',
    '@typescript-eslint/sort-type-union-intersection-members': 'off', // added manually on important types
    '@typescript-eslint/triple-slash-reference': 'error',
    '@typescript-eslint/explicit-member-accessibility': [
      'error',
      { overrides: { constructors: 'off', accessors: 'off' } },
    ],
    'default-case': 'error',
    'dot-notation': 'error',
    'eol-last': ['error', 'always'],
    eqeqeq: 'error',
    'guard-for-in': 'error',
    'import/first': 'error',
    // 'import/no-default-export': 'error',
    'import/no-duplicates': 'error',
    'import/order': [
      'warn',
      {
        groups: [['external', 'builtin'], 'internal', ['parent', 'sibling', 'index']],
        alphabetize: {
          order: 'asc',
        },
      },
    ],
    'max-classes-per-file': ['error', 1],
    'max-lines': ['error', 750],
    'no-bitwise': 'error',
    'no-caller': 'error',
    'no-console': 'warn',
    'no-constant-condition': 'error',
    'no-control-regex': 'error',
    'no-debugger': 'error',
    'no-cond-assign': 'error',
    'no-duplicate-case': 'error',
    'no-empty': [
      'error',
      {
        allowEmptyCatch: true,
      },
    ],
    'no-empty-character-class': 'error',
    'no-ex-assign': 'error',
    'no-extra-boolean-cast': 'error',
    'no-eval': 'error',
    'no-fallthrough': 'error',
    'no-inner-declarations': 'error',
    'no-invalid-regexp': 'error',
    'no-invalid-this': 'off', // must be "off" for @typescript-eslint/no-invalid-this to work
    'no-labels': 'error',
    'no-new-wrappers': 'error',
    'no-regex-spaces': 'error',
    'no-sparse-arrays': 'error',
    'no-unsafe-finally': 'error',
    'no-unused-expressions': [
      'warn',
      {
        allowShortCircuit: true,
        allowTernary: true,
      },
    ],
    'no-var': 'error',
    'object-shorthand': 'error',
    'one-var': ['error', 'never'],
    'prefer-arrow-callback': 'error',
    'prefer-const': 'error',
    'prefer-template': 'error',
    radix: 'error',
    'use-isnan': 'error',
    'react-hooks/exhaustive-deps': 'error',
    'react-hooks/rules-of-hooks': 'error',
    'react/jsx-curly-brace-presence': ['error', { props: 'never' }],
    'react/jsx-key': 'error',
    'react/jsx-no-duplicate-props': 'error',
    'react/jsx-no-target-blank': 'error',
    'react/no-danger-with-children': 'error',
    'react/no-deprecated': 'error',
    'react/no-direct-mutation-state': 'error',
    'react/no-string-refs': 'error',
    'react/no-typos': 'error',
    'react/self-closing-comp': 'error',
    'react/jsx-uses-react': 'off',
    'react/react-in-jsx-scope': 'off',
    'sort-imports': [
      'warn',
      {
        ignoreCase: true,
        ignoreDeclarationSort: true,
      },
    ],
    'spaced-comment': [
      'error',
      'always',
      {
        line: {
          exceptions: ['/'],
        },
        block: {
          balanced: true,
          exceptions: ['*'],
        },
      },
    ],
    'typescript-sort-keys/string-enum': 'off', // added manually on important enums
  },
};
