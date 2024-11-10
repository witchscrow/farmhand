<script lang="ts">
	import Alert from '$lib/components/Alert.svelte';
	import Hls from 'hls.js';
	import { onMount } from 'svelte';

	let { data } = $props();
	let status = $derived(data.video.status);
	// eslint-disable-next-line
	let videoElement: HTMLVideoElement;

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

	onMount(() => {
		if (data.video && data.video.status !== 'Processing') {
			if (Hls.isSupported()) {
				const hls = new Hls();
				hls.loadSource(data.video.playlist);
				hls.attachMedia(videoElement);
				hls.on(Hls.Events.MANIFEST_PARSED, () => {
					videoElement.play().catch((error) => {
						console.log('Playback failed:', error);
					});
				});
			} else if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
				videoElement.src = data.video.playlist;
				videoElement.addEventListener('loadedmetadata', () => {
					videoElement.play().catch((error) => {
						console.log('Playback failed:', error);
					});
				});
			}
		}
	});
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

	<div class="aspect-video relative w-full overflow-hidden rounded-sm bg-primary-900/50">
		{#if data.video}
			{#if status === 'Processing'}
				<div class="flex h-full items-center justify-center">
					<p class="text-primary-200">Please wait while we process your video...</p>
				</div>
			{:else if status === 'Failed'}
				<div class="flex h-full items-center justify-center">
					<p class="text-primary-200">This video failed to process</p>
				</div>
			{:else}
				<video
					bind:this={videoElement}
					class="bg-surface-100 h-full w-full border-8 border-secondary-300 shadow-xl dark:border-primary-900 dark:bg-primary-900"
					controls
					playsinline
					autoplay
				>
					<track kind="captions" />
					Your browser does not support the video tag.
				</video>
			{/if}
		{:else}
			<div class="absolute inset-0 flex items-center justify-center">
				<p class="text-primary-200">No video stream available</p>
			</div>
		{/if}
	</div>
</section>
