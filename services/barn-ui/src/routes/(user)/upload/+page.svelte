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

	const CHUNK_SIZE = 5 * 1024 * 1024; // 5MB chunks

	function handleFileSelect(event: Event): void {
		const target = event.target as HTMLInputElement;
		const files = target?.files;
		if (files) {
			file = files[0];
		}
	}

	async function uploadSequentially(parts: PartUrl[], file: File) {
		const completedParts: Array<{ number: number; etag?: string }> = [];
		const totalParts = parts.length;

		// Process parts one at a time
		for (let i = 0; i < totalParts; i++) {
			const { part_number, url } = parts[i];
			const start = i * CHUNK_SIZE;
			const end = Math.min(start + CHUNK_SIZE, file.size);
			const chunk = file.slice(start, end);

			// Add retry logic
			let attempts = 0;
			const maxAttempts = 3;

			while (attempts < maxAttempts) {
				try {
					const response = await fetch(url, {
						method: 'PUT',
						body: chunk
					});

					if (!response.ok) {
						throw new Error(`Failed to upload part ${part_number}`);
					}

					const etag = response.headers.get('ETag')?.replaceAll('"', '');

					completedParts.push({
						number: part_number,
						etag
					});

					// Update progress
					uploadProgress = ((i + 1) / totalParts) * 100;

					break; // Success, exit retry loop
				} catch (error) {
					attempts++;
					if (attempts === maxAttempts) {
						throw error; // Rethrow if all attempts failed
					}
					// Wait before retrying
					await new Promise((resolve) => setTimeout(resolve, 1000 * attempts));
				}
			}
		}

		return completedParts;
	}

	async function uploadParallel(parts: PartUrl[], file: File, concurrency = 3) {
		const completedParts: Array<{ number: number; etag?: string }> = [];
		const totalParts = parts.length;
		let processedParts = 0;

		// Process parts in batches
		for (let i = 0; i < totalParts; i += concurrency) {
			const batch = parts.slice(i, i + concurrency);
			const uploadPromises = batch.map(async ({ part_number, url }) => {
				const start = (part_number - 1) * CHUNK_SIZE;
				const end = Math.min(start + CHUNK_SIZE, file.size);
				const chunk = file.slice(start, end);

				// Add retry logic
				let attempts = 0;
				const maxAttempts = 3;

				while (attempts < maxAttempts) {
					try {
						const response = await fetch(url, {
							method: 'PUT',
							body: chunk
						});

						if (!response.ok) {
							throw new Error(`Failed to upload part ${part_number}`);
						}

						const etag = response.headers.get('ETag')?.replaceAll('"', '');

						return {
							number: part_number,
							etag
						};
					} catch (error) {
						attempts++;
						if (attempts === maxAttempts) {
							throw error; // Rethrow if all attempts failed
						}
						// Wait before retrying
						await new Promise((resolve) => setTimeout(resolve, 1000 * attempts));
					}
				}
				throw new Error(`Failed to upload part ${part_number} after all attempts`);
			});

			// Wait for all parts in the current batch to complete
			const results = await Promise.all(uploadPromises);
			completedParts.push(...results);

			// Update progress
			processedParts += batch.length;
			uploadProgress = (processedParts / totalParts) * 100;
		}

		// Sort completed parts by part number
		completedParts.sort((a, b) => a.number - b.number);

		return completedParts;
	}

	async function handleUpload() {
		if (!file) return;

		isUploading = true;
		error = null;

		try {
			// Calculate number of parts
			const parts = Math.ceil(file.size / CHUNK_SIZE);

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
			error = err instanceof Error ? err.message : 'Upload failed';
			console.error(err);
		} finally {
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
			<div class="space-y-2">
				<ProgressBar
					value={uploadProgress}
					max={100}
					meter="bg-primary-500"
					track="bg-primary-100"
				/>
				<p class="text-center text-sm">{Math.round(uploadProgress)}% uploaded</p>
			</div>
		{/if}

		{#if error}
			<div class="alert variant-filled-error">
				{error}
			</div>
		{/if}

		<button type="submit" class="variant-filled-primary btn w-full" disabled={!file || isUploading}>
			{isUploading ? 'Uploading...' : 'Upload Video'}
		</button>
	</form>
</div>
