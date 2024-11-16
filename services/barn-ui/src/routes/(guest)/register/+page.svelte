<script lang="ts">
	import Form from '$lib/components/Form.svelte';
	import Input from '$lib/components/Input.svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<Form
	title="Register"
	subtitle="Register for a farmhand account"
	submitText="Register"
	loadingText="Registering..."
	error={form?.error}
>
	<Input label="Username" name="username" type="text" value={form?.username ?? ''} />
	<Input label="Email" name="email" type="email" value={form?.email ?? ''} />
	<Input label="Password" name="password" type="password" />
	<Input label="Confirm Password" name="passwordConfirmation" type="password" />
</Form>
