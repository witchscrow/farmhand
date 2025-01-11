<script lang="ts">
	import Form from '$lib/components/Form.svelte';
	import Input from '$lib/components/Input.svelte';
	import BrandButton from '$lib/components/BrandButton.svelte';
	import type { ActionData, PageData } from './$types';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Throbber from '$lib/components/Throbber.svelte';

	let { form, data } = $props<{ form: ActionData; data: PageData }>();
	let isRedirecting = $state(false);

	onMount(() => {
		if (!data.loggedIn) return;

		isRedirecting = true;
		const interval = setInterval(async () => {
			try {
				if (data?.loggedIn) {
					clearInterval(interval);
					await goto('/', { invalidateAll: true });
				}
			} catch (error) {
				console.error('Login redirect error:', error);
				isRedirecting = false;
				clearInterval(interval);
			}
		}, 250);

		return () => clearInterval(interval);
	});

	function handleTwitchLogin() {
		try {
			goto('/auth/twitch');
		} catch (error) {
			console.error('Twitch login error:', error);
			// Could add error state handling here if needed
		}
	}
</script>

{#if isRedirecting || data.loggedIn}
	<section
		class="flex min-h-[400px] w-full flex-col items-center justify-center space-y-4"
		role="status"
		aria-live="polite"
	>
		<Throbber width="w-10" />
		<p class="text-sm">Logging you in...</p>
	</section>
{:else}
	<section class="flex flex-col items-center space-y-4">
		<header class="flex flex-col space-y-4 text-center">
			<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">Login</h1>
			<p class="text-secondary-800 dark:text-primary-100">Login to your farmhand account</p>
		</header>

		<BrandButton brand="twitch" onclick={handleTwitchLogin} />

		<Form submitText="Login" loadingText="Logging in..." error={form?.error?.message}>
			<Input label="Username" name="username" type="text" value={form?.data?.username ?? ''} />
			<Input label="Password" name="password" type="password" />
		</Form>
	</section>
{/if}
