<script lang="ts">
	import Alert from '$lib/components/Alert.svelte';
	import { enhance } from '$app/forms';
	import type { ActionResult } from '@sveltejs/kit';

	export let form: ActionResult;
	let isLoading = false;
</script>

<form
	method="POST"
	use:enhance={() => {
		isLoading = true;
		return async ({ update }) => {
			await update();
			isLoading = false;
		};
	}}
	class="mx-auto mt-8 max-w-sm rounded-lg bg-white p-6 shadow-md dark:bg-secondary-950 dark:shadow-xl"
>
	{#if form?.error}
		<Alert type="error" message={form.error} />
	{/if}

	<label class="mb-4 block">
		<span class="text-sm text-secondary-800 dark:text-secondary-100">Username</span>
		<input
			name="username"
			type="text"
			value={form?.username ?? ''}
			class="mt-1 block w-full
                   rounded-md border-secondary-200 bg-white
                   text-base shadow-sm
                   focus:border-accent-500 focus:ring focus:ring-accent-200 focus:ring-opacity-50
                   dark:border-secondary-800 dark:bg-secondary-900 dark:text-secondary-100
                   dark:focus:border-accent-400 dark:focus:ring-accent-900"
		/>
	</label>

	<label class="mb-4 block">
		<span class="text-sm text-secondary-800 dark:text-secondary-100">Password</span>
		<input
			name="password"
			type="password"
			class="mt-1 block w-full
                   rounded-md border-secondary-200 bg-white
                   text-base shadow-sm
                   focus:border-accent-500 focus:ring focus:ring-accent-200 focus:ring-opacity-50
                   dark:border-secondary-800 dark:bg-secondary-900 dark:text-secondary-100
                   dark:focus:border-accent-400 dark:focus:ring-accent-900"
		/>
	</label>

	<button
		type="submit"
		disabled={isLoading}
		class="flex w-full items-center
               justify-center gap-2 rounded-md
               bg-accent-500
               px-4 py-2 text-sm font-medium
               text-white hover:bg-accent-600 focus:outline-none focus:ring-2
               focus:ring-accent-400 focus:ring-offset-2 disabled:cursor-not-allowed
               disabled:opacity-50 dark:bg-accent-600
               dark:hover:bg-accent-500
               dark:focus:ring-accent-300 dark:focus:ring-offset-secondary-950"
	>
		{#if isLoading}
			<span>Logging in...</span>
		{:else}
			<span>Login</span>
		{/if}
	</button>
</form>
