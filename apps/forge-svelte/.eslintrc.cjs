module.exports = {
	root: true,
	env: { browser: true, es2021: true },
	parser: '@typescript-eslint/parser',
	parserOptions: { ecmaVersion: 'latest', sourceType: 'module' },
	plugins: ['svelte', '@typescript-eslint'],
	extends: [
		'eslint:recommended',
		'plugin:svelte/recommended',
		'plugin:@typescript-eslint/recommended',
		'prettier'
	],
	overrides: [
		{
			files: ['**/*.svelte'],
			parser: 'svelte-eslint-parser',
			parserOptions: {
				parser: '@typescript-eslint/parser',
				extraFileExtensions: ['.svelte']
			},
			rules: {
				// Не ругаться на самозакрывающиеся компоненты
				'svelte/no-at-html-tags': 'off'
			}
		}
	],
	rules: {
		// Временно смягчаем шум
		'no-unused-vars': 'off',
		'@typescript-eslint/no-unused-vars': [
			'warn',
			{ argsIgnorePattern: '^_', varsIgnorePattern: '^_' }
		],
		'@typescript-eslint/ban-ts-comment': 'off',
		'svelte/no-at-html-tags': 'off'
	}
};
