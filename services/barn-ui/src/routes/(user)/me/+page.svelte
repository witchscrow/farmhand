<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import { page } from '$app/stores';
	import { redirect } from '@sveltejs/kit';

	if (!$page.data.user) {
		redirect(303, '/login');
	}

	const user = $page.data.user;
</script>

<section class="flex flex-col items-center justify-center space-y-10">
	<aside class="flex w-full flex-col space-y-4 text-center">
		<h1
			class="nowrap flex flex-col text-center font-serif text-2xl text-primary-700 dark:text-primary-500"
		>
			<span class="text-xl">Welcome</span> <span class="uppercase">{user.username}</span>
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
					>{user.role}</span
				>
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
		<form slot="content" class="flex flex-col space-y-4">
			<div class="flex items-center justify-between">
				<div>
					<label for="toggleStream" class="font-semibold text-primary-700 dark:text-white">
						Stream Status
						<p class="text-sm text-primary-400">Track when stream starts and stops</p>
					</label>
				</div>
				<input type="checkbox" class="checkbox checked:border-primary-500" name="streamStatus" />
			</div>
			<div class="flex items-center justify-between">
				<div>
					<label for="toggleChat" class="font-semibold text-primary-700 dark:text-white">
						Chat Messages
						<p class="text-sm text-primary-400">Access Twitch chat messages</p>
					</label>
				</div>
				<input type="checkbox" class="checkbox checked:border-primary-500" name="chatMessages" />
			</div>
			<div class="flex items-center justify-between">
				<div>
					<label for="togglePoints" class="font-semibold text-primary-700 dark:text-white">
						Channel Points
						<p class="text-sm text-primary-400">Manage channel point rewards</p>
					</label>
				</div>
				<input type="checkbox" class="checkbox checked:border-primary-500" name="channelPoints" />
			</div>
			<div class="flex items-center justify-between">
				<div>
					<label for="toggleSubs" class="font-semibold text-primary-700 dark:text-white">
						Follows & Subs
						<p class="text-sm text-primary-400">Track follower and subscriber events</p>
					</label>
				</div>
				<input type="checkbox" class="checkbox checked:border-primary-500" name="followsSubs" />
			</div>
			<button class="variant-filled-primary btn w-full">Save Integration Settings</button>
		</form>
	</Card>
</section>
