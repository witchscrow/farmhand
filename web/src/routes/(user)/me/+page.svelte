<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';
	import type { ActionResult } from '@sveltejs/kit';

	let { data, form } = $props<{ data: PageData; form: ActionData }>();

	let message = $state<string | null>(null);
	let { user } = data;

	function clearMessage() {
		message = null;
	}

	const submitEnhance = () => {
		return async ({ result }: { result: ActionResult }) => {
			if (result.type === 'success') {
				message = 'Settings saved successfully';
			} else if (result.type === 'failure') {
				message = 'Failed to save settings';
			} else {
				message = 'An unexpected error occurred';
			}
			setTimeout(() => (message = null), 3000);
		};
	};

	function getSettingValue(settingKey: keyof typeof user.settings): boolean {
		if (!user.settings) return false;
		return user.settings[settingKey] !== null;
	}
</script>

<section class="flex flex-col items-center justify-center space-y-10">
	<aside class="flex w-full flex-col space-y-4 text-center">
		<h1
			class="nowrap flex flex-col text-center font-serif text-2xl text-primary-700 dark:text-primary-500"
		>
			<span class="text-xl">Welcome</span>
			<span class="uppercase">{user.username}</span>
		</h1>
		<p class="text-lg font-semibold text-primary-700 dark:text-primary-100">
			Manage your account here
		</p>
	</aside>

	<Card>
		<div slot="header">
			<h3 class="font-serif text-lg text-primary-700 dark:text-primary-100">Your Account</h3>
			<p class="text-sm text-primary-400 dark:text-primary-200">
				Basic information of you, the user
			</p>
		</div>
		<div slot="content" class="flex justify-between">
			<div>
				<p class="text-lg font-semibold text-primary-700 dark:text-white">{user.username}</p>
				<p class="text-primary-400 dark:text-white">{user.email}</p>
			</div>
			<div class="flex flex-col items-end">
				<p class="text-xs text-primary-400 dark:text-white">Role</p>
				<span
					class="rounded-full bg-primary-100 font-medium text-primary-800 dark:bg-primary-800 dark:text-primary-100"
				>
					{user.role}
				</span>
			</div>
		</div>
	</Card>

	<Card>
		<div slot="header">
			<h3 class="font-serif text-lg text-primary-700 dark:text-primary-100">Twitch Integration</h3>
			<p class="text-sm text-primary-400 dark:text-primary-200">
				Configure your Twitch API connections
			</p>
		</div>
		<form
			slot="content"
			class="flex flex-col space-y-4"
			method="POST"
			action="?/updateTwitchSettings"
			use:enhance={submitEnhance}
		>
			<div class="flex items-center justify-between">
				<div>
					<label for="streamStatus" class="font-semibold text-primary-700 dark:text-white">
						Stream Status
						<p class="text-sm text-primary-400">Track when stream starts and stops</p>
					</label>
				</div>
				<input
					type="checkbox"
					id="streamStatus"
					class="checkbox checked:border-primary-500"
					name="streamStatus"
					checked={getSettingValue('stream_status_enabled')}
				/>
			</div>

			<div class="flex items-center justify-between">
				<div>
					<label for="chatMessages" class="font-semibold text-primary-700 dark:text-white">
						Chat Messages
						<p class="text-sm text-primary-400">Access Twitch chat messages</p>
					</label>
				</div>
				<input
					type="checkbox"
					id="chatMessages"
					class="checkbox checked:border-primary-500"
					name="chatMessages"
					checked={getSettingValue('chat_messages_enabled')}
				/>
			</div>

			<div class="flex items-center justify-between">
				<div>
					<label for="channelPoints" class="font-semibold text-primary-700 dark:text-white">
						Channel Points
						<p class="text-sm text-primary-400">Manage channel point rewards</p>
					</label>
				</div>
				<input
					type="checkbox"
					id="channelPoints"
					class="checkbox checked:border-primary-500"
					name="channelPoints"
					checked={getSettingValue('channel_points_enabled')}
				/>
			</div>

			<div class="flex items-center justify-between">
				<div>
					<label for="followsSubs" class="font-semibold text-primary-700 dark:text-white">
						Follows & Subs
						<p class="text-sm text-primary-400">Track follower and subscriber events</p>
					</label>
				</div>
				<input
					type="checkbox"
					id="followsSubs"
					class="checkbox checked:border-primary-500"
					name="followsSubs"
					checked={getSettingValue('follows_subs_enabled')}
				/>
			</div>

			{#if message}
				<p
					class="text-center text-sm"
					class:text-green-500={message.includes('success')}
					class:text-red-500={message.includes('Failed')}
				>
					{message}
				</p>
			{/if}

			<button class="variant-filled-primary btn w-full" type="submit">
				Save Integration Settings
			</button>
		</form>
	</Card>
</section>
