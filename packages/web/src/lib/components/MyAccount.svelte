<script lang="ts">
	import { onMount } from 'svelte';
	import { computePosition, flip, shift, offset } from '@floating-ui/dom';
	import type { ComputePositionReturn } from '@floating-ui/dom';
	import Button from './Button.svelte';
	import SignOut from './icons/SignOut.svelte';
	import Chevron from './icons/Chevron.svelte';
	import MyAccount from './icons/MyAccount.svelte';
	import { enhance } from '$app/forms';
	import type { User } from '$lib/stores/user';

	let hideMenu: (ev?: Event) => void;
	let account: HTMLElement | null;
	let menu: HTMLElement | null;

	let { user }: { user: User | null } = $props();

	onMount(() => {
		account = document.getElementById('account');
		menu = document.getElementById('menu');

		function update(el: HTMLElement) {
			if (!menu) return;

			computePosition(el, menu, {
				placement: 'top',
				middleware: [offset(6), flip(), shift({ padding: 5 })]
			}).then(({ x, y }: ComputePositionReturn) => {
				if (!menu) return;
				Object.assign(menu.style, {
					left: `${x}px`,
					top: `${y}px`
				});
			});
		}

		function showMenu(ev: MouseEvent) {
			if (!menu) return;
			menu.style.display = 'block';

			const targetElement = ev.currentTarget as HTMLElement;
			const element = document.getElementById(targetElement.id);
			if (element) {
				update(element);
			}
		}

		hideMenu = () => {
			if (!menu) return;
			menu.style.display = '';
		};

		if (account) {
			account.addEventListener('click', showMenu);
		}
	});

	function clickOutside(node: HTMLElement) {
		const handleClick = (event: MouseEvent) => {
			const target = event.target as Node;
			if (node && !node.contains(target) && !event.defaultPrevented) {
				node.dispatchEvent(new CustomEvent('clickoutside'));
			}
		};

		document.addEventListener('click', handleClick, true);

		return {
			destroy() {
				document.removeEventListener('click', handleClick, true);
			}
		};
	}
</script>

{#if user}
	<Button id="account">
		<span>Account</span>
		<Chevron
			class="ml-2 transition-transform {menu?.style.display === 'block' ? 'rotate-180' : ''}"
		/>
	</Button>

	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		id="menu"
		role="menu"
		use:clickOutside
		on:clickoutside={hideMenu}
		on:click={hideMenu}
		tabindex="0"
		class="min-w-52 rounded border-2 border-primary-800 bg-primary-900"
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
