<script lang="ts">
	import '../app.css';
	import { isAuthenticated } from '$lib/stores/auth';
	import type { LayoutData } from './$types';
	import type { Snippet } from 'svelte';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>

<main class="flex min-h-screen flex-col">
	<Header user={data.user} />
	<section class="mx-auto min-h-full w-full max-w-screen-lg flex-grow p-4">
		{@render children()}
	</section>
	<Footer />
</main>
