<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import { user } from '$lib/user';
	import { redirect } from '@sveltejs/kit';

	if (!$user) throw redirect(307, '/login');
</script>

<section class="flex flex-col items-center justify-center space-y-10">
	<aside class="flex w-full flex-col space-y-4 text-center">
		<h1 class="font-serif text-3xl text-primary-800">Welcome {$user.username}</h1>
		<p>Manage your account here</p>
	</aside>
	<Card>
		<div slot="header">
			<h3 class="font-serif text-lg text-primary-700">Your Account</h3>
			<p class="text-sm text-gray-700">Basic information of you, the user</p>
		</div>
		<div slot="content" class="space-y-1">
			<p class="text-lg font-semibold">{$user.username}</p>
			<p class="text-gray-700">{$user.email}</p>
		</div>
	</Card>
</section>
