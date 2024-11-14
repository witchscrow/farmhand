<script lang="ts">
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
</script>

<section class="mx-auto max-w-4xl space-y-6 p-4">
	<div class="space-y-2 text-center">
		<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">My Videos</h1>
		<p class="text-secondary-800 dark:text-primary-100">Manage your uploaded videos</p>
	</div>

	{#if data.videos && data.videos.length > 0}
		<div class="grid gap-4">
			{#each data.videos as video}
				<div
					class="relative grid grid-cols-4 rounded-lg border border-primary-200/20 bg-secondary-300 p-6 shadow-lg backdrop-blur-sm transition-all hover:bg-secondary-400 dark:border-primary-900/40 dark:bg-primary-800 dark:hover:bg-primary-900"
				>
					<a href="/watch?v={video.id}" class="col-span-3 flex flex-col items-start space-y-2">
						<div class="flex items-start justify-between">
							<h2 class="text-lg font-medium text-black dark:text-white">
								{video.title}
							</h2>
						</div>
						<p class="text-sm">
							Status: {video.status}
						</p>
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
	{/if}
</section>
