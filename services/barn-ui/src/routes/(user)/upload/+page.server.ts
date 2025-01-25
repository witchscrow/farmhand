import type { Actions, PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';
import { redirect, fail } from '@sveltejs/kit';

export const load = (async ({ cookies }) => {
	const token = cookies.get('jwt');
	if (!token) {
		throw redirect(303, '/login');
	}
	return {};
}) satisfies PageServerLoad;

export const actions = {
	initUpload: async ({ request, cookies }) => {
		const token = cookies.get('jwt');
		const formData = await request.formData();
		const title = formData.get('title')?.toString();
		const fileName = formData.get('fileName')?.toString();
		const fileType = formData.get('fileType')?.toString();
		const parts = parseInt(formData.get('parts')?.toString() || '0');

		if (!fileName || !fileType || !parts) {
			return fail(400, { error: 'Missing required upload information' });
		}

		try {
			const response = await fetch(`${env.API_URL}/upload/start`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Cookie: `jwt=${token}`
				},
				body: JSON.stringify({
					parts,
					key: fileName,
					content_type: fileType,
					title: title || undefined
				})
			});

			if (!response.ok) {
				return fail(response.status, { error: 'Failed to initialize upload' });
			}

			const data = await response.json();
			return { success: true, data };
		} catch (error) {
			console.error('Upload initialization failed:', error);
			return fail(500, { error: 'Failed to initialize upload' });
		}
	},

	completeUpload: async ({ request, cookies }) => {
		const token = cookies.get('jwt');
		const formData = await request.formData();
		const upload_id = formData.get('upload_id')?.toString();
		const video_id = formData.get('video_id')?.toString();
		const key = formData.get('key')?.toString();
		const completed_parts = JSON.parse(formData.get('completed_parts')?.toString() || '[]');

		try {
			const response = await fetch(`${env.API_URL}/upload/complete`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Cookie: `jwt=${token}`
				},
				body: JSON.stringify({
					upload_id,
					video_id,
					key,
					completed_parts
				})
			});

			if (!response.ok) {
				return fail(response.status, { error: 'Failed to complete upload' });
			}

			return { success: true };
		} catch (error) {
			console.error('Upload completion failed:', error);
			return fail(500, { error: 'Failed to complete upload' });
		}
	}
} satisfies Actions;
