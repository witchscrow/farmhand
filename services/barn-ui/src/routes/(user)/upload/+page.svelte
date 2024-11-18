<script lang="ts">
	import Alert from '$lib/components/Alert.svelte';
	import { onDestroy } from 'svelte';
	import { filesize } from 'filesize';

	let file: File | null = $state(null);
	let progress = $state(0);
	let uploading = $state(false);
	let paused = $state(false);
	let abortController: AbortController | null = $state(null);
	let errorMessage: string | null = $state(null);
	let successMessage: string | null = $state(null);
	let { data } = $props();

	const CHUNK_SIZE = 50 * 1024 * 1024; // 50MB chunks
	const MAX_CONCURRENT_UPLOADS = 5;
	const MAX_RETRIES = 3;
	const COMPRESSION_THRESHOLD = 5 * 1024 * 1024; // 5MB

	async function calculateChecksum(chunk: Blob): Promise<string> {
		const arrayBuffer = await chunk.arrayBuffer();
		const hashBuffer = await crypto.subtle.digest('SHA-256', arrayBuffer);
		const hashArray = Array.from(new Uint8Array(hashBuffer));
		return hashArray.map((b) => b.toString(16).padStart(2, '0')).join('');
	}

	async function compressChunk(chunk: Blob): Promise<Blob> {
		if (chunk.size < COMPRESSION_THRESHOLD) {
			return chunk;
		}

		try {
			if ('CompressionStream' in window) {
				const compressed = new Blob([chunk]).stream().pipeThrough(new CompressionStream('gzip'));
				return new Blob([await new Response(compressed).blob()], {
					type: chunk.type
				});
			}
		} catch (error) {
			console.warn('Compression failed, using original chunk:', error);
		}
		return chunk;
	}

	async function uploadChunk(chunk: Blob, filename: string, chunkIndex: number, totalSize: number) {
		abortController = new AbortController();

		// For MP4 files, skip compression to maintain file integrity
		const shouldCompress = !filename.toLowerCase().endsWith('.mp4');
		const processedChunk = shouldCompress ? await compressChunk(chunk) : chunk;
		const checksum = await calculateChecksum(processedChunk);
		const totalChunks = Math.ceil(totalSize / CHUNK_SIZE);

		console.log(`Uploading chunk details:`, {
			chunkIndex,
			chunkSize: processedChunk.size,
			totalSize,
			totalChunks,
			isLastChunk: chunkIndex === totalChunks - 1,
			isMP4: filename.toLowerCase().endsWith('.mp4')
		});

		const formData = new FormData();
		formData.append('chunkIndex', chunkIndex.toString());
		formData.append('totalSize', totalSize.toString());
		formData.append('checksum', checksum);
		formData.append('file', processedChunk, filename);
		formData.append(
			'compressed',
			(shouldCompress && chunk.size !== processedChunk.size).toString()
		);

		const response = await fetch(data.apiUrl, {
			method: 'POST',
			body: formData,
			signal: abortController?.signal,
			headers: {
				Authorization: `Bearer ${data.token}`
			}
		});

		if (!response.ok) {
			const errorText = await response.text();
			throw new Error(`Upload failed: ${errorText}`);
		}

		return response;
	}

	async function uploadChunkWithRetry(
		chunk: Blob,
		filename: string,
		chunkIndex: number,
		totalSize: number
	): Promise<Response> {
		let lastError: Error | null = null;

		for (let attempt = 0; attempt < MAX_RETRIES; attempt++) {
			try {
				return await uploadChunk(chunk, filename, chunkIndex, totalSize);
			} catch (error) {
				lastError = error as Error;
				if (error instanceof Error && error.name === 'AbortError') {
					throw error;
				}
				console.warn(`Attempt ${attempt + 1} failed for chunk ${chunkIndex}:`, error);
				if (attempt < MAX_RETRIES - 1) {
					const delay = Math.min(1000 * Math.pow(2, attempt), 10000);
					await new Promise((resolve) => setTimeout(resolve, delay));
				}
			}
		}
		throw lastError || new Error('Upload failed after all retries');
	}

	async function uploadFile(selectedFile: File) {
		uploading = true;
		progress = 0;
		errorMessage = null;
		successMessage = null;

		const totalChunks = Math.ceil(selectedFile.size / CHUNK_SIZE);
		let completedChunks = 0;

		const isMP4 = selectedFile.name.toLowerCase().endsWith('.mp4');

		console.log(`File details:`, {
			name: selectedFile.name,
			size: selectedFile.size,
			totalChunks,
			chunkSize: CHUNK_SIZE,
			isMP4
		});

		try {
			const chunks = [];
			let chunkIndex = 0;
			for (let start = 0; start < selectedFile.size; start += CHUNK_SIZE) {
				const end = Math.min(start + CHUNK_SIZE, selectedFile.size);
				chunks.push({
					chunk: selectedFile.slice(start, end, selectedFile.type),
					index: chunkIndex++
				});
			}

			// For MP4 files, use semi-sequential uploads with controlled concurrency
			if (isMP4) {
				const windowSize = 3; // Number of chunks to upload in parallel for MP4
				for (let i = 0; i < chunks.length && !paused; i += windowSize) {
					const batch = chunks.slice(i, i + windowSize);
					const uploadPromises = batch.map(({ chunk, index }) =>
						uploadChunkWithRetry(chunk, selectedFile.name, index, selectedFile.size)
					);

					await Promise.all(uploadPromises);
					completedChunks += batch.length;
					progress = (completedChunks / totalChunks) * 100;

					console.log(
						`Completed ${completedChunks}/${totalChunks} chunks (${progress.toFixed(2)}%)`
					);
				}
			} else {
				// For non-MP4 files, use full parallel uploads
				for (let i = 0; i < chunks.length && !paused; i += MAX_CONCURRENT_UPLOADS) {
					const batch = chunks.slice(i, i + MAX_CONCURRENT_UPLOADS);
					const uploadPromises = batch.map(({ chunk, index }) =>
						uploadChunkWithRetry(chunk, selectedFile.name, index, selectedFile.size)
					);

					await Promise.all(uploadPromises);
					completedChunks += batch.length;
					progress = (completedChunks / totalChunks) * 100;
				}
			}

			if (!paused) {
				if (completedChunks !== totalChunks) {
					throw new Error(`Upload incomplete: ${completedChunks}/${totalChunks} chunks uploaded`);
				}

				uploading = false;
				progress = 100;
				successMessage = `Successfully uploaded ${selectedFile.name}`;
			}
		} catch (error) {
			console.error('Upload error:', error);
			if (error instanceof Error) {
				if (error.name === 'AbortError') {
					console.log('Upload paused');
				} else {
					errorMessage = error.message;
					uploading = false;
				}
			} else {
				errorMessage = 'An unknown error occurred';
				uploading = false;
			}
		}
	}

	function handleFileSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files[0]) {
			const selectedFile = input.files[0];
			if (!selectedFile.type.startsWith('video/')) {
				errorMessage = 'Please select a video file';
				return;
			}

			const validExtensions = ['.mp4', '.mov', '.m4v'];
			const hasValidExtension = validExtensions.some((ext) =>
				selectedFile.name.toLowerCase().endsWith(ext)
			);

			if (!hasValidExtension) {
				errorMessage = 'Only .mp4 and .mov files are allowed';
				return;
			}
			file = selectedFile;
			errorMessage = null;
		}
	}

	function togglePause() {
		paused = !paused;
		if (!paused && file) {
			uploadFile(file);
		} else if (abortController) {
			abortController.abort();
		}
	}

	onDestroy(() => {
		if (abortController) {
			abortController.abort();
		}
	});
</script>

<section class="flex flex-col items-center justify-center">
	<aside class="flex flex-col space-y-4 text-center">
		<h1 class="font-serif text-2xl text-secondary-700 dark:text-primary-500">Upload</h1>
		<p class="text-secondary-800 dark:text-primary-100">Upload your latest livestream or replay</p>
		<Alert
			type="info"
			message="Don't want to manually upload your livestreams? Try our program for auto-uploads"
		/>

		<!-- Upload UI -->
		<div class="flex w-full flex-col space-y-4">
			<label
				class="flex cursor-pointer flex-col items-center rounded-lg border border-primary-200 bg-white px-4 py-6 tracking-wide text-primary-800 shadow-lg transition-colors hover:bg-primary-50 dark:border-primary-900 dark:bg-primary-800 dark:text-primary-200 dark:hover:bg-primary-900"
			>
				<svg
					class="h-8 w-8"
					fill="currentColor"
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 20 20"
				>
					<path
						d="M16.88 9.1A4 4 0 0 1 16 17H5a5 5 0 0 1-1-9.9V7a3 3 0 0 1 4.52-2.59A4.98 4.98 0 0 1 17 8c0 .38-.04.74-.12 1.1zM11 11h3l-4-4-4 4h3v3h2v-3z"
					/>
				</svg>
				<span class="mt-2 text-base">Select a video file (.mp4 or .mov)</span>
				<input
					type="file"
					class="hidden"
					accept="video/mp4,video/quicktime"
					onchange={handleFileSelect}
					disabled={uploading}
				/>
			</label>

			{#if file}
				<div class="mt-4 rounded-lg bg-white p-4 shadow dark:bg-primary-900">
					<p class="font-medium">Selected file: {file.name}</p>
					<p class="text-sm">Size: {filesize(file.size)}</p>

					{#if !uploading}
						<button
							class="mt-4 rounded bg-primary-600 px-4 py-2 text-white transition-colors hover:bg-primary-700"
							onclick={() => file && uploadFile(file)}
						>
							Start Upload
						</button>
					{/if}
				</div>
			{/if}

			{#if uploading}
				<div class="mt-4 space-y-2">
					<div class="h-2.5 w-full rounded-full bg-gray-200">
						<div
							class="h-2.5 rounded-full bg-primary-600 transition-all duration-300"
							style="width: {progress}%"
						></div>
					</div>
					<div class="flex justify-between text-sm text-gray-600">
						<span>{progress.toFixed(1)}%</span>
						<button class="text-primary-600 hover:text-primary-800" onclick={togglePause}>
							{paused ? 'Resume' : 'Pause'}
						</button>
					</div>
				</div>
			{/if}

			{#if errorMessage}
				<Alert type="error" message={errorMessage} />
			{/if}

			{#if successMessage}
				<Alert type="success" message={successMessage} />
			{/if}
		</div>
	</aside>
</section>
