<script lang="ts">
	import { deserialize } from '$app/forms';
	import type { ActionData } from './$types';
	import { FileDropzone, ProgressBar } from '@skeletonlabs/skeleton';
	interface PartUrl {
		part_number: number;
		url: string;
	}

	interface UploadInitResponse {
		upload_id: string;
		video_id: string;
		key: string;
		part_urls: PartUrl[];
	}

	let file: File | null = $state(null);
	let uploadProgress = $state(0);
	let isUploading = $state(false);
	let error: string | null = $state(null);
	let title = $state('');
	let uploadSpeed = $state(0);
	let currentUpload: AbortController | null = $state(null);

	const CHUNK_MB = 5 * 1024 * 1024;

	function handleFileSelect(event: Event): void {
		const target = event.target as HTMLInputElement;
		const files = target?.files;
		if (files) {
			file = files[0];
		}
	}

	function getOptimalConcurrency(fileSize: number): number {
		// Start with more conservative concurrency
		if (fileSize > 1024 * 1024 * 1024) {
			// > 1GB
			return 5;
		} else if (fileSize > 100 * 1024 * 1024) {
			// > 100MB
			return 2;
		}
		return 2; // Default to 2 concurrent uploads
	}

	const getOptimalChunkSize = (fileSize: number): number => {
		// Use smaller chunks to reduce connection issues
		if (fileSize > 1024 * 1024 * 1024) {
			// > 1GB
			return 10 * 1024 * 1024; // 10MB for large files
		}
		return 5 * 1024 * 1024; // 5MB default
	};

	async function uploadChunk(
		{ part_number, url }: PartUrl,
		chunk: Blob
	): Promise<{ number: number; etag?: string }> {
		let attempts = 0;
		const maxAttempts = 5; // Increase max attempts
		const initialBackoffMS = 1000;

		while (attempts < maxAttempts) {
			try {
				// Add timeout to the fetch request
				const controller = new AbortController();
				const timeoutId = setTimeout(() => controller.abort(), 30000); // 30 second timeout

				const response = await fetch(url, {
					method: 'PUT',
					body: chunk,
					headers: {
						'Content-Length': chunk.size.toString(),
						'Content-Type': 'application/octet-stream'
					},
					signal: controller.signal
				});

				clearTimeout(timeoutId);

				if (!response.ok) {
					throw new Error(`Failed to upload part ${part_number}: ${response.statusText}`);
				}

				const etag = response.headers.get('ETag')?.replaceAll('"', '');
				return { number: part_number, etag };
			} catch (error) {
				attempts++;
				console.log(`Attempt ${attempts} failed for part ${part_number}:`, error);

				if (attempts === maxAttempts) {
					throw error;
				}

				// Exponential backoff with jitter
				const backoffTime = initialBackoffMS * Math.pow(2, attempts - 1);
				const jitter = Math.random() * 1000;
				await new Promise((resolve) => setTimeout(resolve, backoffTime + jitter));
			}
		}
		throw new Error(`Failed to upload part ${part_number} after ${maxAttempts} attempts`);
	}

	async function uploadParallel(parts: PartUrl[], file: File) {
		const chunkSize = getOptimalChunkSize(file.size);
		let concurrency = getOptimalConcurrency(file.size);
		const completedParts: Array<{ number: number; etag?: string }> = [];
		const totalParts = parts.length;
		let processedParts = 0;
		let consecutiveFailures = 0;

		const uploadedBytes = new Map<number, number>();
		const startTime = Date.now();

		// Process in batches with dynamic concurrency
		for (let i = 0; i < totalParts; ) {
			const batch = parts.slice(i, i + concurrency);
			const batchUploads = batch.map(async ({ part_number, url }) => {
				const start = (part_number - 1) * chunkSize;
				const end = Math.min(start + chunkSize, file.size);
				const chunk = file.slice(start, end);

				try {
					const result = await uploadChunk({ part_number, url }, chunk);
					uploadedBytes.set(part_number, chunk.size);

					// Calculate and update upload speed
					const totalUploaded = Array.from(uploadedBytes.values()).reduce((a, b) => a + b, 0);
					const elapsedSeconds = (Date.now() - startTime) / 1000;
					uploadSpeed = totalUploaded / (1024 * 1024) / elapsedSeconds;

					consecutiveFailures = 0; // Reset failure counter on success
					return result;
				} catch (error) {
					consecutiveFailures++;
					throw error;
				}
			});

			try {
				const results = await Promise.all(batchUploads);
				completedParts.push(...results);
				processedParts += batch.length;
				uploadProgress = (processedParts / totalParts) * 100;
				i += batch.length;
			} catch (error) {
				console.error(`Batch upload failed:`, error);

				// Reduce concurrency after consecutive failures
				if (consecutiveFailures >= 2 && concurrency > 1) {
					concurrency--;
					console.log(`Reduced concurrency to ${concurrency} due to consecutive failures`);
					consecutiveFailures = 0;
				}

				// Wait longer between retries if we're having issues
				await new Promise((resolve) => setTimeout(resolve, 2000));

				// Don't increment i, so we retry the failed batch
				continue;
			}

			// Add a small delay between batches to prevent overwhelming the connection
			await new Promise((resolve) => setTimeout(resolve, 500));
		}

		return completedParts.sort((a, b) => a.number - b.number);
	}

	async function handleUpload() {
		if (!file) return;

		currentUpload = new AbortController();
		isUploading = true;
		error = null;

		try {
			// Calculate number of parts
			const parts = Math.ceil(file.size / CHUNK_MB);

			// Initialize upload through server action
			const formData = new FormData();
			formData.append('title', title);
			formData.append('fileName', file.name);
			formData.append('fileType', file.type);
			formData.append('parts', parts.toString());

			const response = await fetch('?/initUpload', {
				method: 'POST',
				body: formData
			});

			const result: ActionData = deserialize(await response.text());
			if (result?.error || !result) {
				throw new Error(result?.error);
			}

			const { upload_id, video_id, key, part_urls } = result.data as UploadInitResponse;
			// Upload parts in controlled batches
			const completedParts = await uploadParallel(part_urls, file);
			// Complete the upload through server action
			const completeForm = new FormData();
			completeForm.append('upload_id', upload_id);
			completeForm.append('video_id', video_id);
			completeForm.append('key', key);
			completeForm.append('completed_parts', JSON.stringify(completedParts));

			const completeResponse = await fetch('?/completeUpload', {
				method: 'POST',
				body: completeForm
			});

			const completeResult = await completeResponse.json();
			if (completeResult.error) {
				throw new Error(completeResult.error);
			}

			// Reset form
			file = null;
			title = '';
			uploadProgress = 0;
			console.log('Uploaded video ', video_id);
		} catch (err) {
			// @ts-ignore
			if (err.name === 'AbortError') {
				error = 'Upload cancelled';
			} else {
				error = err instanceof Error ? err.message : 'Upload failed';
			}
		} finally {
			currentUpload = null;
			isUploading = false;
		}
	}
</script>

<div class="container mx-auto max-w-2xl space-y-4 p-4">
	<h1 class="h1">Upload Video</h1>

	<form onsubmit={handleUpload} class="card space-y-4 p-4">
		<label class="label">
			<span>Title (optional)</span>
			<input
				class="input"
				type="text"
				bind:value={title}
				disabled={isUploading}
				placeholder="Enter video title"
			/>
		</label>

		<FileDropzone name="file" accept="video/*" disabled={isUploading} onchange={handleFileSelect}>
			{#snippet lead()}
				<i class="fas fa-cloud-upload-alt text-4xl"></i>
			{/snippet}
			{#snippet message()}
				<div class="flex flex-col justify-center space-y-2 divide-y-2 divide-surface-500">
					<div class="flex flex-col items-center justify-center">
						<span class="font-bold">Upload Video</span>
						<span class="text-sm">Drag and drop or click to select</span>
					</div>
					{#if file}
						<span class="text-sm font-bold">{file.name}</span>
					{/if}
				</div>
			{/snippet}
		</FileDropzone>

		{#if isUploading}
			<div class="space-y-4 divide-y-2 divide-surface-500">
				<!-- Upload Progress -->
				<div>
					<div class="flex flex-nowrap items-center justify-between py-4">
						<p class="text-sm">Upload Progress</p>
						<p class="text-lg font-semibold">{Math.round(uploadProgress)}% uploaded</p>
					</div>
					<ProgressBar
						value={uploadProgress}
						max={100}
						meter="bg-primary-500"
						track="bg-primary-300"
					/>
				</div>

				<!-- Speed Progress -->
				<div class="justify-star flex flex-nowrap items-center justify-between pt-4">
					<p class="text-sm">Upload Speed</p>
					<p class="text-lg font-semibold">{uploadSpeed.toFixed(1)} MB/s</p>
				</div>
			</div>
		{/if}

		{#if error}
			<div class="alert variant-filled-error">
				{error}
			</div>
		{/if}

		<button
			type="submit"
			class="btn w-full {isUploading ? 'variant-filled-error' : 'variant-filled-primary'} {!file &&
				'variant-soft-primary'}"
			disabled={!file}
		>
			{isUploading ? 'Cancel Video' : 'Upload Video'}
		</button>
	</form>
</div>
