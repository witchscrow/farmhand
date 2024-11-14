<script lang="ts">
	import Button from './Button.svelte';
	import SignOut from './icons/SignOut.svelte';
	import Chevron from './icons/Chevron.svelte';
	import MyAccount from './icons/MyAccount.svelte';
	import { enhance } from '$app/forms';
	import type { User } from '$lib/stores/user';
	import { popup } from '@skeletonlabs/skeleton';
	import type { PopupSettings } from '@skeletonlabs/skeleton';
	import Video from './icons/Video.svelte';

	let { user }: { user: User | null } = $props();
	let isOpen = $state(false);
	const popupHover: PopupSettings = {
		event: 'click',
		target: 'my-account',
		placement: 'bottom',
		state: (e: Record<string, boolean>) => (isOpen = e.state)
	};

	const menuItems = [
		{
			copy: 'My Account',
			icon: MyAccount,
			url: '/me'
		},
		{
			copy: 'My Videos',
			icon: Video,
			url: '/me/videos'
		}
	];
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
		class="divide z-20 min-w-52 divide-y-2 divide-primary-900 rounded border-2 border-primary-700 bg-primary-800 shadow-xl"
	>
		<aside class="flex flex-col items-start px-6 py-4 text-primary-50 dark:text-white">
			<p class="text-xs font-medium">Signed in as</p>
			<p class="text-lg font-semibold">{user.username}</p>
			<p class="text-base text-primary-200 dark:text-primary-200">{user.email}</p>
		</aside>
		<ul class="p-2">
			{#each menuItems as item}
				<li>
					<a
						href={item.url}
						class="flex w-full flex-nowrap items-center rounded px-4 py-2 font-semibold text-primary-50 hover:bg-primary-700 dark:text-white"
					>
						{#if item.icon}
							<item.icon class="mr-2" />
						{/if}
						<span>{item.copy}</span>
					</a>
				</li>
			{/each}
			<li>
				<form action="/?/logout" method="POST" use:enhance>
					<button
						class="flex w-full flex-nowrap items-center rounded px-4 py-2 font-semibold text-primary-50 hover:bg-primary-700 dark:text-white"
					>
						<SignOut class="mr-2" />
						<span>Sign out</span>
					</button>
				</form>
			</li>
		</ul>
	</div>
{/if}
