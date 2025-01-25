<script lang="ts">
	import type { ActionData } from './$types';

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

	let file: File | null = null;
	let uploadProgress = 0;
	let isUploading = false;
	let error: string | null = null;
	let title = '';

	const CHUNK_SIZE = 5 * 1024 * 1024; // 5MB chunks

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

			const result = await response.json();
			if (result.error) {
				throw new Error(result.error);
			}

			const { upload_id, video_id, key, part_urls } = result.data as UploadInitResponse;

			// Upload parts in parallel directly to presigned URLs
			const completedParts = await Promise.all(
				part_urls.map(async ({ part_number, url }, index) => {
					// Check if file is still available
					if (!file) {
						throw new Error('File was removed during upload');
					}
					const start = index * CHUNK_SIZE;
					const end = Math.min(start + CHUNK_SIZE, file.size);
					const chunk = file.slice(start, end);

					const response = await fetch(url, {
						method: 'PUT',
						body: chunk
					});

					if (!response.ok) {
						throw new Error(`Failed to upload part ${part_number}`);
					}

					const etag = response.headers.get('ETag')?.replaceAll('"', '');
					if (!etag) throw new Error('No ETag received');

					// Update progress
					uploadProgress = ((index + 1) / parts) * 100;

					return {
						number: part_number,
						etag
					};
				})
			);

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
			alert('Upload successful! Video is being processed.');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Upload failed';
			console.error(err);
		} finally {
			isUploading = false;
		}
	}
</script>

<div class="mx-auto max-w-2xl p-6">
	<h1 class="mb-8 text-3xl font-bold">Upload Video</h1>

	<form on:submit|preventDefault={handleUpload} class="space-y-6">
		<div>
			<label for="title" class="mb-2 block text-sm font-medium text-gray-700">
				Title (optional)
			</label>
			<input
				type="text"
				id="title"
				bind:value={title}
				disabled={isUploading}
				class="w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
			/>
		</div>

		<div>
			<label for="file" class="mb-2 block text-sm font-medium text-gray-700"> Video File </label>
			<input
				type="file"
				id="file"
				accept="video/*"
				on:change={(e) => (file = e.currentTarget.files?.[0] || null)}
				disabled={isUploading}
				class="w-full text-sm text-gray-500 file:mr-4 file:rounded-md file:border-0 file:bg-blue-50 file:px-4 file:py-2 file:text-sm file:font-semibold file:text-blue-700 hover:file:bg-blue-100"
			/>
		</div>

		{#if isUploading}
			<div class="h-2.5 w-full rounded-full bg-gray-200">
				<div
					class="h-2.5 rounded-full bg-blue-600 transition-all duration-300"
					style="width: {uploadProgress}%"
				></div>
			</div>
			<p class="text-center text-sm text-gray-600">
				{Math.round(uploadProgress)}% uploaded
			</p>
		{/if}

		{#if error}
			<div class="rounded-md border border-red-200 bg-red-50 px-4 py-3 text-red-700">
				{error}
			</div>
		{/if}

		<button
			type="submit"
			disabled={!file || isUploading}
			class="flex w-full justify-center rounded-md border border-transparent bg-blue-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:bg-gray-300"
		>
			{isUploading ? 'Uploading...' : 'Upload Video'}
		</button>
	</form>
</div>
