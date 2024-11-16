<script lang="ts">
	import '../app.css';
	import type { Snippet } from 'svelte';
	import type { LayoutServerData } from './$types';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	let { children }: { children: Snippet; data: LayoutServerData } = $props();
</script>

<main class="flex min-h-screen flex-col">
	{@render children()}
</main>
