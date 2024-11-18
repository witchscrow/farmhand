<script lang="ts">
	import Alert from './Alert.svelte';
	import Throbber from './Throbber.svelte';
	import Button from './Button.svelte';
	import { enhance } from '$app/forms';

	export let title: string = '';
	export let subtitle: string = '';
	export let submitText: string = 'Submit';
	export let loadingText: string = 'Submitting...';
	export let error: string | null = null;

	let isSubmitting = false;

	function handleSubmit() {
		isSubmitting = true;
		return async ({ update }: { update: () => Promise<void> }) => {
			await update();
			isSubmitting = false;
		};
	}
</script>

<section class="flex w-full flex-col items-center justify-center">
	{#if title || subtitle}
		<aside class="flex flex-col space-y-4 text-center">
			<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">{title}</h1>
			{#if subtitle}
				<p class="text-secondary-800 dark:text-primary-100">{subtitle}</p>
			{/if}
		</aside>
	{/if}

	<form
		method="POST"
		use:enhance={handleSubmit}
		class="mt-8 w-full max-w-sm flex-grow rounded border-2 border-secondary-900 bg-white p-6 shadow-md dark:border-primary-800 dark:bg-primary-900 dark:shadow-xl"
	>
		{#if error}
			<Alert type="error" message={error} />
		{/if}

		<slot />

		<Button type="submit" disabled={isSubmitting} variant="secondary" class="w-full py-2">
			{#if isSubmitting}
				<Throbber />
				<span class="w-full text-center">{loadingText}</span>
			{:else}
				<span class="w-full text-center">{submitText}</span>
			{/if}
		</Button>
	</form>
</section>
