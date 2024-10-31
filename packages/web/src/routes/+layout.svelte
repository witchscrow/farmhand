<script lang="ts">
	import '../app.css';
	import { isAuthenticated } from '$lib/stores/auth';
	import type { LayoutData } from './$types';
	import type { Snippet } from 'svelte';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';
	import Header from '$lib/components/Header.svelte';
</script>

<Header />
<main>
	{#if isAuthenticated}
		<pre>{JSON.stringify(data, null, 2)}</pre>
	{/if}
	<div class="fixed right-4 top-4 z-50">
		<ThemeToggler />
	</div>
	{@render children()}
</main>
