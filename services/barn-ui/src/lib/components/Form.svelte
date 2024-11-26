<script lang="ts">
	import Alert from './Alert.svelte';
	import Throbber from './Throbber.svelte';
	import Button from './Button.svelte';
	import { enhance } from '$app/forms';

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
	<form
		method="POST"
		use:enhance={handleSubmit}
		class="mt-8 w-full max-w-sm flex-grow rounded border-2 border-secondary-900 bg-white p-6 shadow-md dark:border-primary-800 dark:bg-primary-900 dark:shadow-xl"
	>
		{#if error}
			<Alert type="error" message={error} class="mb-4" />
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
