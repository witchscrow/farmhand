<script lang="ts">
	import type { Component } from 'svelte';
	import Twitch from '$lib/components/icons/Twitch.svelte';
	import Email from './icons/Email.svelte';

	type Brand = 'twitch' | 'email';
	interface Props {
		brand: Brand;
		prefix?: string;
		onclick?: (event: MouseEvent) => void;
	}

	let { brand, onclick = () => {}, prefix = 'Login with ' }: Props = $props();

	type BrandSettings = {
		classes: string;
		Icon: Component;
		name: string;
	};
	const getBrandSettings = (brand: Brand): BrandSettings => {
		switch (brand) {
			case 'twitch': {
				return {
					Icon: Twitch,
					classes:
						'border-brand-twitch bg-white dark:bg-black text-black dark:text-white hover:text-brand-twitch',
					name: 'Twitch'
				};
			}
			case 'email':
			default: {
				return {
					Icon: Email,
					classes: 'border-primary-500 bg-primary-700 hover:text-primary-500',
					name: 'Email'
				};
			}
		}
	};

	const { Icon, name, classes } = getBrandSettings(brand);
</script>

<button
	class="flex flex-nowrap items-center justify-center gap-2 rounded border-4 px-4 py-2 text-sm font-semibold transition-all {classes}"
	{onclick}
>
	{`${prefix} ${name}`}
	<Icon />
</button>
