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
		<p>Manage your account here</p>
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
</section>
