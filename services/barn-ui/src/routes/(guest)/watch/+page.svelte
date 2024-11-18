<script lang="ts">
	import Alert from '$lib/components/Alert.svelte';
	import VideoPlayer from '$lib/components/VideoPlayer.svelte';

	let { data } = $props();
	let status = $derived(data.video.status);

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

<section class="mx-auto max-w-screen-2xl space-y-6 p-4">
	<div class="space-y-2 text-center">
		<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">Watch</h1>
		<p class="text-secondary-800 dark:text-primary-100">
			Stream your content with our high-performance video player
		</p>
	</div>

	{#if data.video && status !== 'Completed'}
		<Alert
			type={getAlertType(status)}
			message={status === 'Processing'
				? 'Your video is currently being processed. This usually takes a few minutes depending on the file size.'
				: status === 'Failed'
					? 'This video failed to process. Please try uploading it again.'
					: 'Loading video information...'}
		/>
	{/if}

	<VideoPlayer video={data.video} />
</section>
