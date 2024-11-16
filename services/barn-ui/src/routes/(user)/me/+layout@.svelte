<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutServerData } from './$types';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import MyAccount from '$lib/components/MyAccount.svelte';
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';

	let { children, data }: { children: Snippet; data: LayoutServerData } = $props();
</script>

<main class="grid grid-cols-12 grid-rows-main">
	<Header
		width="w-full max-w-none"
		class="col-span-12 row-start-1 border-b-2 border-secondary-400 bg-surface-500/30 dark:border-surface-500"
	>
		{#snippet actions()}
			<nav class="mr-4 flex justify-evenly space-x-4">
				<MyAccount user={data.user} />
				<ThemeToggler />
			</nav>
		{/snippet}
	</Header>
	<aside class="col-span-2 row-start-2">This is where the sidebar goes</aside>
	<section class="col-span-10 row-start-2 mx-auto min-h-full w-full flex-grow p-4">
		{@render children()}
	</section>
	<Footer class="col-span-12 row-start-3" />
</main>
