<script lang="ts">
	import Hls from 'hls.js';
	import { onMount } from 'svelte';

	let { video }: { video: { status: string; playlist?: string } } = $props();

	// eslint-disable-next-line
	let videoElement: HTMLVideoElement;

	onMount(() => {
		if (video && video.playlist && video.status !== 'Processing') {
			if (Hls.isSupported()) {
				const hls = new Hls();
				hls.loadSource(video.playlist);
				hls.attachMedia(videoElement);
				hls.on(Hls.Events.MANIFEST_PARSED, () => {
					videoElement.play().catch((error) => {
						console.log('Playback failed:', error);
					});
				});
			} else if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
				videoElement.src = video.playlist;
				videoElement.addEventListener('loadedmetadata', () => {
					videoElement.play().catch((error) => {
						console.log('Playback failed:', error);
					});
				});
			}
		}
	});
</script>

<div class="aspect-video relative w-full overflow-hidden rounded-sm bg-primary-900/50">
	{#if video}
		{#if video.status === 'Processing'}
			<div class="flex h-full items-center justify-center">
				<p class="text-primary-200">Please wait while we process your video...</p>
			</div>
		{:else if video.status === 'Failed'}
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
