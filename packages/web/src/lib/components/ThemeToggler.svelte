<script lang="ts">
	import { onMount } from 'svelte';

	let theme: 'light' | 'dark';

	onMount(() => {
		// Get initial theme from localStorage or system preference
		const savedTheme = localStorage.getItem('theme');
		// @ts-expect-error should fallback to dark if nothing is set
		theme =
			savedTheme ?? (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
		document.body.className = theme;
	});

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
		document.body.className = theme;
		localStorage.setItem('theme', theme);
	}
</script>

<button
	type="button"
	class="text-primary-800 hover:bg-primary-100 focus:ring-primary-200 dark:text-primary-200 dark:hover:bg-primary-700 dark:focus:ring-primary-700 rounded-lg p-2.5 text-sm focus:outline-none focus:ring-4"
	on:click={toggleTheme}
>
	{#if theme === 'dark'}
		<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
			<path
				d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"
			/>
		</svg>
	{:else}
		<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
			<path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
		</svg>
	{/if}
</button>
