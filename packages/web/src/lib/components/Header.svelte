<script lang="ts">
	import Button from './Button.svelte';
	import Logo from './Logo.svelte';
	import MyAccount from './MyAccount.svelte';
	import ThemeToggler from './ThemeToggler.svelte';
	import { user } from '$lib/user';
</script>

<header class="flex w-full items-center justify-center text-black dark:text-white">
	<nav class="flex w-full max-w-screen-xl items-center justify-between py-4 text-sm">
		<a href="/" class="flex flex-nowrap items-center">
			<Logo size={50} />
			<span class="font-semibold">FARMHAND</span>
		</a>
		<!-- You can put additional top menu items here -->
		<aside class="flex items-center space-x-2">
			{#if $user}
				<MyAccount />
			{:else}
				<Button href="/login">Log in</Button>
				<Button href="/register">Register</Button>
			{/if}
			<ThemeToggler />
		</aside>
	</nav>
</header>
