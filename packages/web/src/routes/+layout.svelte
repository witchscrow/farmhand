<script lang="ts">
	import '../app.css';
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';
	import Header from '$lib/components/Header.svelte';
	let { children } = $props();
</script>

<Header />
<main>
	<div class="fixed right-4 top-4 z-50">
		<ThemeToggler />
	</div>
	{@render children()}
</main>
