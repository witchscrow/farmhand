<script lang="ts">
	import Form from '$lib/components/Form.svelte';
	import Input from '$lib/components/Input.svelte';
	import BrandButton from '$lib/components/BrandButton.svelte';
	import type { ActionData } from './$types';

	let { form }: { form: ActionData } = $props();
	const prefix = 'Register with';
</script>

<section class="flex w-full flex-col items-center justify-center space-y-4">
	<aside class="flex flex-col space-y-4 text-center">
		<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">Register</h1>
		<p class="text-secondary-800 dark:text-primary-100">Register for a farmhand account</p>
	</aside>
	<BrandButton brand="twitch" {prefix} />
</section>
