<script lang="ts">
	import Form from '$lib/components/Form.svelte';
	import Input from '$lib/components/Input.svelte';
	import LoginWithButton from '$lib/components/LoginWithButton.svelte';
	import type { ActionData } from './$types';

	let { form }: { form: ActionData } = $props();
	const prefix = 'Register with';
</script>

<section class="flex flex-col items-center justify-center space-y-4">
	<aside class="flex flex-col space-y-4 text-center">
		<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">Register</h1>
		<p class="text-secondary-800 dark:text-primary-100">Register for a farmhand account</p>
	</aside>
	<LoginWithButton brand="twitch" {prefix} />
	<LoginWithButton brand="email" {prefix} />
	<Form submitText="Register" loadingText="Registering..." error={form?.error}>
		<Input label="Username" name="username" type="text" value={form?.username ?? ''} />
		<Input label="Email" name="email" type="email" value={form?.email ?? ''} />
		<Input label="Password" name="password" type="password" />
		<Input label="Confirm Password" name="passwordConfirmation" type="password" />
	</Form>
</section>
