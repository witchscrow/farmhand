<script lang="ts">
	import type { User } from '$lib/stores/user';
	import Button from './Button.svelte';
	import Logo from './Logo.svelte';
	import MyAccount from './MyAccount.svelte';
	import ThemeToggler from './ThemeToggler.svelte';
	let { user }: { user: User | null } = $props();
</script>

<header class="flex w-full items-center justify-center text-black dark:text-white">
	<nav class="flex w-full max-w-screen-xl items-center justify-between py-4 text-sm">
		<div class="justify-even-10 flex flex-nowrap items-center space-x-10">
			<a href="/" class="flex flex-nowrap items-center">
				<Logo size={50} />
				<span class="font-semibold">FARMHAND</span>
			</a>
			<a href="/videos">Videos</a>
		</div>
		<aside class="flex items-center space-x-2">
			{#if user}
				<Button variant="secondary" href="/upload">Upload</Button>
				<MyAccount {user} />
			{:else}
				<Button href="/login">Log in</Button>
				<Button href="/register">Register</Button>
			{/if}
			<ThemeToggler />
		</aside>
	</nav>
</header>
