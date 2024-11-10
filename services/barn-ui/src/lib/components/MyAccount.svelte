<script lang="ts">
	import Button from './Button.svelte';
	import SignOut from './icons/SignOut.svelte';
	import Chevron from './icons/Chevron.svelte';
	import MyAccount from './icons/MyAccount.svelte';
	import { enhance } from '$app/forms';
	import type { User } from '$lib/stores/user';
	import { popup } from '@skeletonlabs/skeleton';
	import type { PopupSettings } from '@skeletonlabs/skeleton';

	let { user }: { user: User | null } = $props();
	let isOpen = $state(false);
	const popupHover: PopupSettings = {
		event: 'click',
		target: 'my-account',
		placement: 'bottom',
		state: (e: Record<string, boolean>) => (isOpen = e.state)
	};
</script>

{#if user}
	<div use:popup={popupHover}>
		<Button class="space-x-2">
			<span>Account</span>
			<Chevron class="{isOpen ? 'rotate-180' : 'rotate-0'} transition-transform" />
		</Button>
	</div>
	<div
		data-popup="my-account"
		class="z-20 min-w-52 rounded border-2 border-primary-800 bg-primary-900"
	>
		<aside class="px-6 py-4 text-primary-50 dark:text-white">
			<p class="text-xs font-medium">Signed in as</p>
			<p class="text-lg font-semibold">{user.username}</p>
			<p class="text-base text-primary-200 dark:text-primary-200">{user.email}</p>
		</aside>
		<hr class="border-primary-800" />
		<ul class="p-2">
			<li>
				<a
					href="/account"
					class="flex w-full flex-nowrap items-center rounded px-4 py-2 font-semibold text-primary-50 hover:bg-primary-800 dark:text-white"
				>
					<MyAccount class="mr-2" />
					<span>My Account</span>
				</a>
			</li>
			<li>
				<form action="?/logout" method="POST" use:enhance>
					<button
						class="flex w-full flex-nowrap items-center rounded px-4 py-2 font-semibold text-primary-50 hover:bg-primary-800 dark:text-white"
					>
						<SignOut class="mr-2" />
						<span>Sign out</span>
					</button>
				</form>
			</li>
		</ul>
	</div>

	<style>
		#menu {
			display: none;
			width: max-content;
			position: absolute;
			top: 0;
			left: 0;
		}
	</style>
{/if}
