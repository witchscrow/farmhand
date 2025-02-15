<script lang="ts">
	import Alert from './Alert.svelte';
	import Throbber from './Throbber.svelte';
	import Button from './Button.svelte';
	import { enhance } from '$app/forms';

	export let submitText: string = 'Submit';
	export let loadingText: string = 'Submitting...';
	export let error: string | null = null;

	let isSubmitting = false;
	let formError: string | null = null;

	function handleSubmit() {
		isSubmitting = true;
		formError = null; // Reset error state before submission

		return async ({
			update,
			result
		}: {
			update: () => Promise<void>;
			result: { type: string; error?: { message: string } };
		}) => {
			try {
				await update();

				if (result.type === 'error') {
					formError = result.error?.message || 'An unexpected error occurred';
				}
			} catch (err) {
				formError = err instanceof Error ? err.message : 'An unexpected error occurred';
			} finally {
				isSubmitting = false;
			}
		};
	}
</script>

<section class="flex w-full flex-col items-center justify-center">
	<form
		method="POST"
		use:enhance={handleSubmit}
		class="mt-8 w-full max-w-sm flex-grow rounded border-2 border-secondary-900 bg-white p-6 shadow-md dark:border-primary-800 dark:bg-primary-900 dark:shadow-xl"
	>
		{#if error || formError}
			<Alert
				type="error"
				message={error || formError || 'There was an error submitting, try again'}
				class="mb-4"
			/>
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
