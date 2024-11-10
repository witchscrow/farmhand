<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		variant?: 'primary' | 'secondary';
		children: Snippet;
		href?: string;
		class?: string;
		type?: 'button' | 'submit' | 'reset' | null | undefined;
	}
	let {
		variant = 'primary',
		href,
		children,
		class: className = '',
		type = 'button',
		...restProps
	}: Props = $props();
</script>

{#if href}
	<a
		{href}
		class="{className} inline-flex items-center justify-between rounded-sm border border-b-4 px-4 py-1 text-sm font-medium transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 disabled:pointer-events-none disabled:opacity-50 {variant ===
		'primary'
			? 'border-primary-900 bg-primary-700 text-white'
			: 'border-secondary-600 bg-secondary-200 text-black'} "
		{...restProps}
	>
		{@render children()}
	</a>
{:else}
	<button
		{type}
		class="{className} inline-flex items-center justify-between rounded-sm border border-b-4 px-4 py-1 text-sm font-medium transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 disabled:pointer-events-none disabled:opacity-50 {variant ===
		'primary'
			? 'border-primary-900 bg-primary-700 text-white'
			: 'border-secondary-600 bg-secondary-200 text-black'}"
		{...restProps}
	>
		{@render children()}
	</button>
{/if}
