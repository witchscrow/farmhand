<script lang="ts">
	import Form from '$lib/components/Form.svelte';
	import Input from '$lib/components/Input.svelte';
	import type { ActionResult } from '@sveltejs/kit';
	export let form: ActionResult;
</script>

<Form
	title="Login"
	subtitle="Login to your farmhand account"
	submitText="Login"
	loadingText="Logging in..."
	error={form?.error}
>
	<Input label="Username" name="username" type="text" value={form?.username ?? ''} />
	<Input label="Password" name="password" type="password" />
</Form>
