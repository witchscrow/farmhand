<script lang="ts">
	import Table from '$lib/components/Table.svelte';
	import type { Snippet } from 'svelte';

	function _formatDate(dateString: string) {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	type VideoID = string;
	type Video = { id: VideoID; title: string; thumbnail: string };
	let videos: Video[] = [
		{ id: 'vid1', title: 'My First Video', thumbnail: '' },
		{ id: 'vid2', title: 'How to Cook', thumbnail: '' },
		{ id: 'vid3', title: 'Gaming Tutorial', thumbnail: '' },
		{ id: 'vid4', title: 'Vacation Vlog', thumbnail: '' },
		{ id: 'vid5', title: 'Product Review', thumbnail: '' }
	];
	const columns = ['ID', 'Thumbnail', 'Title'];
	let selected = $state<VideoID[]>([]);
	const toggleSelected = (id: VideoID) => {
		if (selected?.includes(id)) {
			selected = selected.filter((selectedId) => selectedId !== id);
		} else {
			selected = [...(selected || []), id];
		}
	};
</script>

<section class="flex w-full flex-col space-y-6 p-4">
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
	<div class="btn-group self-end">
		<button class="variant-filled-error" disabled={selected.length === 0}>Delete</button>
	</div>
	<Table {columns}>
		{#snippet rows()}
			{#each videos as video}
				<tr
					class={selected.includes(video.id) ? 'table-row-checked' : ''}
					onclick={() => toggleSelected(video.id)}
				>
					<td class="table-cell h-full !align-middle">
						<input type="checkbox" class="checkbox" checked={selected.includes(video.id)} />
					</td>
					<td>
						<img
							src="http://placeskull.com/170/95"
							alt="Temporary skull placeholder for thumbnail"
							class="w-min"
						/>
					</td>
					<td class="!align-middle">
						<p>{video.title}</p>
					</td>
				</tr>
			{/each}
		{/snippet}
	</Table>
</section>
