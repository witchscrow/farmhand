<script lang="ts">
	import '../app.css';
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';
	let { children } = $props();
</script>

<main
	class="min-w-screen dark:bg-secondary-950 bg-secondary-50 dark:text-primary-100 text-primary-950 min-h-screen text-base"
>
	<div class="fixed right-4 top-4 z-50">
		<ThemeToggler />
	</div>
	{@render children()}
</main>
