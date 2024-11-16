<script lang="ts">
	import Datatable from '$lib/components/Datatable.svelte';
	import { TableHandler } from '@vincjo/datatables/server';

	function _formatDate(dateString: string) {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	type Video = { title: string };
	const videos: Video[] = [
		{ title: 'Example Video 1' },
		{ title: 'Example Video 2' },
		{ title: 'Example Video 3' },
		{ title: 'Example Video 4' },
		{ title: 'Example Video 5' }
	];
	const table = new TableHandler(videos);
</script>

<section class="w-full space-y-6 p-4">
	<div
		class="dark:bg-primary-950/40 rounded-lg border border-primary-200/20 bg-black/40 p-6 text-center backdrop-blur-sm dark:border-primary-800/40"
	>
		<p class="text-white/80 dark:text-primary-300/80">You haven't uploaded any videos yet</p>
		<a
			href="/upload"
			class="mt-4 inline-block rounded-md bg-primary-600 px-4 py-2 text-sm text-white hover:bg-primary-700"
		>
			Upload a Video
		</a>
	</div>
	<Datatable {table} />
</section>
