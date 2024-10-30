<script lang="ts">
	import Throbber from '$lib/components/Throbber.svelte';
	import Alert from '$lib/components/Alert.svelte';
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let form: ActionData;
	let isSubmitting = false;
</script>

<form
	method="POST"
	use:enhance={() => {
		isSubmitting = true;
		return async ({ update }) => {
			await update();
			isSubmitting = false;
		};
	}}
	class="dark:bg-secondary-950 mx-auto mt-8 max-w-sm rounded-lg bg-white p-6 shadow-md dark:shadow-xl"
>
	{#if form?.error}
		<Alert type="error" message={form.error} />
	{/if}

	<label class="mb-4 block">
		<span class="text-secondary-800 dark:text-secondary-100 text-sm">Username</span>
		<input
			name="username"
			type="text"
			value={form?.username ?? ''}
			class="border-secondary-200 focus:border-accent-500 focus:ring-accent-200
                   dark:border-secondary-800 dark:bg-secondary-900 dark:text-secondary-100
                   dark:focus:border-accent-400 dark:focus:ring-accent-900
                   mt-1 block w-full rounded-md
                   bg-white text-base shadow-sm
                   focus:ring focus:ring-opacity-50"
		/>
	</label>

	<label class="mb-4 block">
		<span class="text-secondary-800 dark:text-secondary-100 text-sm">Email</span>
		<input
			name="email"
			type="email"
			value={form?.email ?? ''}
			class="border-secondary-200 focus:border-accent-500 focus:ring-accent-200
                   dark:border-secondary-800 dark:bg-secondary-900 dark:text-secondary-100
                   dark:focus:border-accent-400 dark:focus:ring-accent-900
                   mt-1 block w-full rounded-md
                   bg-white text-base shadow-sm
                   focus:ring focus:ring-opacity-50"
		/>
	</label>

	<label class="mb-4 block">
		<span class="text-secondary-800 dark:text-secondary-100 text-sm">Password</span>
		<input
			name="password"
			type="password"
			class="border-secondary-200 focus:border-accent-500 focus:ring-accent-200
                   dark:border-secondary-800 dark:bg-secondary-900 dark:text-secondary-100
                   dark:focus:border-accent-400 dark:focus:ring-accent-900
                   mt-1 block w-full rounded-md
                   bg-white text-base shadow-sm
                   focus:ring focus:ring-opacity-50"
		/>
	</label>

	<label class="mb-4 block">
		<span class="text-secondary-800 dark:text-secondary-100 text-sm">Confirm Password</span>
		<input
			name="passwordConfirmation"
			type="password"
			class="border-secondary-200 focus:border-accent-500 focus:ring-accent-200
                   dark:border-secondary-800 dark:bg-secondary-900 dark:text-secondary-100
                   dark:focus:border-accent-400 dark:focus:ring-accent-900
                   mt-1 block w-full rounded-md
                   bg-white text-base shadow-sm
                   focus:ring focus:ring-opacity-50"
		/>
	</label>

	<button
		disabled={isSubmitting}
		class="bg-accent-500 hover:bg-accent-600 focus:ring-accent-400
               dark:bg-accent-600 dark:hover:bg-accent-500 dark:focus:ring-accent-300
               dark:focus:ring-offset-secondary-950
               flex w-full items-center justify-center
               gap-2 rounded-md px-4 py-2
               text-sm font-medium text-white
               focus:outline-none focus:ring-2
               focus:ring-offset-2
               disabled:cursor-not-allowed disabled:opacity-50"
	>
		{#if isSubmitting}
			<Throbber />
			<span>Registering...</span>
		{:else}
			<span>Register</span>
		{/if}
	</button>
</form>
