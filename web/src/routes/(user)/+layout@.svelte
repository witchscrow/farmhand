<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutServerData } from './$types';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import MyAccount from '$lib/components/MyAccount.svelte';
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';
	import Button from '$lib/components/Button.svelte';

	let { children, data }: { children: Snippet; data: LayoutServerData } = $props();
	const sidebarLinks = [
		{
			link: '/dashboard/streams',
			label: 'Streams'
		}
	];
</script>

<main class="grid min-h-screen grid-cols-12 grid-rows-main divide-x divide-y">
	<Header
		width="w-full max-w-none"
		class="dark:bg-primary-950/30 col-span-12 row-start-1 border-primary-200/20 bg-black/40 backdrop-blur-sm dark:border-primary-800/40"
		headerLink="/dashboard"
	>
		{#snippet actions()}
			<nav class="mr-4 flex items-center justify-evenly space-x-4">
				<a href="/dashboard">
					<span class="font-semibold">Dashboard</span>
				</a>
				<MyAccount user={data.user} />
				<ThemeToggler />
			</nav>
		{/snippet}
	</Header>
	<aside
		class="col-span-2 row-start-2 min-h-max border-primary-200/20 bg-black/40 p-4 backdrop-blur-sm dark:border-primary-800/40"
	>
		<nav>
			<ul>
				{#each sidebarLinks as link}
					<li class="font-semibold uppercase">
						<a href={link.link}>{link.label}</a>
					</li>
				{/each}
			</ul>
		</nav>
	</aside>
	<section
		class="col-span-10 row-start-2 mx-auto min-h-full w-full flex-grow border-primary-200/20 bg-black/40 p-4 backdrop-blur-sm dark:border-primary-800/40"
	>
		{@render children()}
	</section>
	<Footer
		class="col-span-12 row-start-3 border-primary-200/20 bg-black/40 backdrop-blur-sm dark:border-primary-800/40"
	/>
</main>
