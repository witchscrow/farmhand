<script lang="ts">
	import Form from '$lib/components/Form.svelte';
	import Input from '$lib/components/Input.svelte';
	import BrandButton from '$lib/components/BrandButton.svelte';
	import type { ActionData } from './$types';
	import { goto } from '$app/navigation';
	let { form }: { form: ActionData } = $props();
</script>

<section class="flex flex-col items-center space-y-4">
	<aside class="flex flex-col space-y-4 text-center">
		<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">Login</h1>
		<p class="text-secondary-800 dark:text-primary-100">Login to your farmhand account</p>
	</aside>
	<BrandButton brand="twitch" onclick={() => goto('/auth/twitch')} />
	<Form submitText="Login" loadingText="Logging in..." error={form?.error}>
		<Input label="Username" name="username" type="text" value={form?.username ?? ''} />
		<Input label="Password" name="password" type="password" />
	</Form>
</section>
