<script lang="ts">
	import Throbber from '$lib/components/Throbber.svelte';
	import Alert from '$lib/components/Alert.svelte';
	import Button from '$lib/components/Button.svelte';
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let form: ActionData;
	let isSubmitting = false;
</script>

<section class="flex flex-col items-center justify-center">
	<aside class="flex flex-col space-y-4 text-center">
		<h1 class="font-serif text-3xl text-primary-800">Register</h1>
		<p>Register for a farmhand account</p>
	</aside>
	<form
		method="POST"
		use:enhance={() => {
			isSubmitting = true;
			return async ({ update }) => {
				await update();
				isSubmitting = false;
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
			<span class="text-sm text-secondary-800 dark:text-primary-100">Email</span>
			<input
				name="email"
				type="email"
				value={form?.email ?? ''}
				class="rounded border-2 text-black placeholder-primary-200 dark:border-primary-950 dark:bg-primary-800 dark:text-primary-50"
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
		<label class="mb-4 flex flex-col justify-start space-y-2">
			<span class="text-sm text-secondary-800 dark:text-secondary-100">Confirm Password</span>
			<input
				name="passwordConfirmation"
				type="password"
				class="rounded border-2 text-black placeholder-primary-200 dark:border-primary-950 dark:bg-primary-800 dark:text-primary-50"
			/>
		</label>

		<Button type="submit" disabled={isSubmitting} class="w-full border-primary-950 py-2">
			{#if isSubmitting}
				<Throbber />
				<span class="w-full text-center">Registering...</span>
			{:else}
				<span class="w-full text-center">Register</span>
			{/if}
		</Button>
	</form>
</section>
