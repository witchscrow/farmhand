<script lang="ts">
	export let variant: 'primary' | 'secondary' = 'primary';
	export let type: 'button' | 'submit' | 'reset' = 'button';
	export let disabled = false;
	export let href = '';
	export let id = '';
</script>

{#if href}
	<a
		{id}
		{href}
		class="{$$restProps.class} inline-flex items-center justify-between rounded-sm border border-b-4 px-4 py-1 text-sm font-medium transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 disabled:pointer-events-none disabled:opacity-50 {variant ===
		'primary'
			? 'border-primary-900 bg-primary-700 text-white'
			: 'border-secondary-600 bg-secondary-200 text-black'} "
		on:click
	>
		<slot />
	</a>
{:else}
	<button
		{id}
		{type}
		{disabled}
		class="{$$restProps.class} inline-flex items-center justify-between rounded-sm border border-b-4 px-4 py-1 text-sm font-medium transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 disabled:pointer-events-none disabled:opacity-50 {variant ===
		'primary'
			? 'border-primary-900 bg-primary-700 text-white'
			: 'border-secondary-600 bg-secondary-200 text-black'}"
		on:click
	>
		<slot />
	</button>
{/if}
