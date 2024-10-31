<script lang="ts">
	import Alert from '$lib/components/Alert.svelte';
	import Throbber from '$lib/components/Throbber.svelte';
	import { enhance } from '$app/forms';
	import type { ActionResult } from '@sveltejs/kit';
	import Button from '$lib/components/Button.svelte';

	export let form: ActionResult;
	let isLoading = false;
</script>

<section class="flex flex-col items-center justify-center">
	<aside class="flex flex-col space-y-4 text-center">
		<h1 class="font-serif text-3xl text-primary-800">Login</h1>
		<p>Login to your farmhand account</p>
	</aside>
	<form
		method="POST"
		use:enhance={() => {
			isLoading = true;
			return async ({ update }) => {
				await update();
				isLoading = false;
			};
		}}
		class="mt-8 w-full max-w-sm flex-grow rounded border-2 border-secondary-950 bg-white p-6 shadow-md dark:border-black dark:bg-primary-900 dark:shadow-xl"
	>
		{#if form?.error}
			<Alert type="error" message={form.error} />
		{/if}

		<label class="mb-4 flex flex-col justify-start space-y-2">
			<span class="text-sm text-secondary-800 dark:text-primary-100">Username</span>
			<input
				class="rounded border-2 text-black placeholder-primary-200 dark:border-primary-950 dark:bg-primary-800 dark:text-primary-50"
				name="username"
				type="text"
				value={form?.username ?? ''}
			/>
		</label>
		<label class="mb-4 flex flex-col justify-start space-y-2">
			<span class="text-sm text-secondary-800 dark:text-secondary-100">Password</span>
			<input
				class="rounded border-2 text-black placeholder-primary-200 dark:border-primary-950 dark:bg-primary-800 dark:text-primary-50"
				name="password"
				type="password"
			/>
		</label>

		<Button type="submit" disabled={isLoading} class="w-full border-primary-950 py-2">
			{#if isLoading}
				<Throbber />
				<span class="w-full text-center">Logging in...</span>
			{:else}
				<span class="w-full text-center">Login</span>
			{/if}
		</Button>
	</form>
</section>
