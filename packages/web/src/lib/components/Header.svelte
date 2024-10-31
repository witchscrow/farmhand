<script lang="ts">
	import Button from './Button.svelte';
	import Logo from './Logo.svelte';
	import MyAccount from './MyAccount.svelte';
	import ThemeToggler from './ThemeToggler.svelte';

	export let user: { username: string; email: string } | null = null;
</script>

<header class="flex w-full items-center justify-center text-black dark:text-white">
	<nav class="flex w-full max-w-screen-lg items-center justify-between py-4 text-sm">
		<ul class="flex items-center justify-start">
			<li>
				<a href="/" class="flex flex-nowrap items-center">
					<Logo size={50} />
				</a>
			</li>
			<li>
				<a href="/about" class="mx-4">About</a>
			</li>
			<li>
				<a href="/features" class="mx-4">Features</a>
			</li>
			<li>
				<a href="/pricing" class="mx-4">Pricing</a>
			</li>
			<li>
				<a href="/contact" class="mx-4">Contact</a>
			</li>
		</ul>
		<aside class="flex items-center space-x-2">
			{#if user}
				<MyAccount {user} />
			{:else}
				<Button href="/login">Log in</Button>
				<Button href="/register">Register</Button>
			{/if}
			<ThemeToggler />
		</aside>
	</nav>
</header>
