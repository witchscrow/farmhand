<script>
	import { page } from '$app/stores';
	import Button from '$lib/components/Button.svelte';
</script>

<section class="flex min-h-[50vh] flex-col items-center justify-center space-y-4 text-center">
	<h1 class="font-serif text-3xl text-primary-700 dark:text-primary-200">
		{$page.status}: {$page.error?.message || 'Page not found'}
	</h1>

	<p class="text-gray-600 dark:text-gray-300">
		Looks like you've wandered into uncharted territory!
	</p>

	<Button href="/">Back to Home</Button>
</section>
