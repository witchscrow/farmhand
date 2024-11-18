<script lang="ts">
	import { applyAction, deserialize } from '$app/forms';
	import { invalidateAll } from '$app/navigation';
	import Table from '$lib/components/Table.svelte';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import type { ActionResult } from '@sveltejs/kit';

	const formatDate = (dateString: string) => {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	};

	type VideoID = string;
	const columns = ['ID', 'Thumbnail', 'Title', 'Published'];
	let selected = $state<VideoID[]>([]);
	const toggleSelected = (id: VideoID) => {
		if (selected?.includes(id)) {
			selected = selected.filter((selectedId) => selectedId !== id);
		} else {
			selected = [...(selected || []), id];
		}
	};

	let { data } = $props();
	let isLoading = $state(false);

	async function handleDelete(event: {
		preventDefault: Event['preventDefault'];
		currentTarget: EventTarget & HTMLFormElement;
	}) {
		event.preventDefault();
		const formData = new FormData();
		selected.forEach((id) => {
			formData.append('video_delete_id_list', id);
		});
		isLoading = true;
		const response = await fetch(event.currentTarget.action, {
			method: 'POST',
			body: formData
		});

		const result: ActionResult = deserialize(await response.text());

		if (result.type === 'success') {
			await invalidateAll();
		}

		applyAction(result);

		isLoading = false;
	}

	const isVideoLoading = (videoID: VideoID) => {
		return selected.includes(videoID) && isLoading;
	};
</script>

<section class="grid w-full auto-rows-max items-center space-y-6 p-4">
	<div class="btn-group ml-auto">
		<form method="POST" onsubmit={handleDelete} action="?/delete">
			<button class="variant-filled-error" disabled={selected.length === 0}>Delete</button>
		</form>
	</div>
	<Table {columns}>
		{#snippet rows()}
			{#if data.videos.length === 0}
				<tr>
					<td colspan="4" class="text-center">
						<p class="p-4 dark:text-surface-100">No videos found</p>
					</td>
				</tr>
			{/if}
			{#each data.videos as video}
				<tr
					class="relative {selected.includes(video.id)
						? '!bg-secondary-500/20 dark:!bg-primary-500/20'
						: ''}"
					onclick={() => toggleSelected(video.id)}
				>
					<td
						class="table-cell h-full !align-middle {isVideoLoading(video.id) ? 'opacity-50' : ''}"
					>
						<input
							type="checkbox"
							class="checkbox checked:border-primary-500"
							checked={selected.includes(video.id)}
						/>
					</td>
					<td class={isVideoLoading(video.id) ? 'opacity-50' : ''}>
						<div class="h-[95px] w-[170px] bg-surface-900">
							<a href="/watch?v={video.id}">
								<img
									src="http://placeskull.com/170/95"
									alt="Temporary skull placeholder for thumbnail"
									class="w-min"
								/>
							</a>
						</div>
					</td>
					<td class="!align-middle {isVideoLoading(video.id) ? 'opacity-50' : ''}">
						<p>{video.title}</p>
					</td>
					<td class="!align-middle {isVideoLoading(video.id) ? 'opacity-50' : ''}">
						<p>{formatDate(video.created_at)}</p>
					</td>
					{#if isVideoLoading(video.id)}
						<td
							class="absolute left-0 top-0 z-10 mx-auto flex h-full w-full items-center justify-center p-20"
						>
							<ProgressRadial
								stroke={80}
								meter="stroke-primary-500"
								track="stroke-primary-500/30"
								strokeLinecap="round"
								width="w-12"
							/>
						</td>
					{/if}
				</tr>
			{/each}
		{/snippet}
	</Table>
</section>
