<script lang="ts">
	import Alert from '$lib/components/Alert.svelte';

	let { data } = $props();

	function formatDate(dateString: string) {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getAlertType(status: string) {
		switch (status) {
			case 'Completed':
				return 'success';
			case 'Processing':
				return 'warning';
			case 'Failed':
				return 'error';
			default:
				return 'info';
		}
	}
</script>

<section class="mx-auto max-w-4xl space-y-6 p-4">
	<div class="space-y-2 text-center">
		<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">Videos</h1>
		<p class="text-secondary-800 dark:text-primary-100">Browse and manage your uploaded videos</p>
	</div>

	{#if data.videos && data.videos.length > 0}
		<div class="grid gap-4">
			{#each data.videos as video}
				<div
					class="relative rounded-lg border border-primary-200/20 bg-secondary-300 p-6 shadow-lg backdrop-blur-sm transition-all hover:bg-secondary-400 dark:border-primary-900/40 dark:bg-primary-800 dark:hover:bg-primary-900"
				>
					<a href="/watch?v={video.id}" class="block">
						<div class="flex items-start justify-between">
							<h2 class="text-lg font-medium text-black dark:text-white">
								{video.title}
							</h2>
							{#if video.status !== 'Completed'}
								<div class="ml-4">
									<Alert type={getAlertType(video.status)} message={video.status} size="small" />
								</div>
							{/if}
						</div>
						<div class="mt-2 text-sm text-secondary-800/80 dark:text-primary-100/80">
							<div>Created: {formatDate(video.created_at)}</div>
							<div>Updated: {formatDate(video.updated_at)}</div>
						</div>
					</a>
				</div>
			{/each}
		</div>
	{:else}
		<div
			class="rounded-lg border border-primary-200/20 bg-black/40 p-6 text-center backdrop-blur-sm dark:border-primary-800/40 dark:bg-primary-950/40"
		>
			<p class="text-white/80 dark:text-primary-300/80">No videos found</p>
		</div>
	{/if}
</section>
