<script lang="ts">
	import '../app.css';
	import type { Snippet } from 'svelte';
	import type { LayoutServerData } from './$types';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	let { children, data }: { children: Snippet; data: LayoutServerData } = $props();
</script>

<main class="flex min-h-screen flex-col">
	<Header user={data.user} />
	<section class="mx-auto min-h-full w-full max-w-screen-lg flex-grow p-4">
		{@render children()}
	</section>
	<Footer />
</main>
