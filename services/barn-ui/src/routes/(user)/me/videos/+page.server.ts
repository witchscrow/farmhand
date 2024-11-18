import type { PageServerLoad, Actions } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { fetchVideos } from '$lib/server/videos';

export const load = (async ({ locals }) => {
	try {
		if (!locals.user) throw redirect(307, '/login');
		const videos = await fetchVideos({ channel: locals.user.username });
		return {
			videos
		};
	} catch (e) {
		throw error(500, `Error fetching videos: ${e}`);
	}
}) satisfies PageServerLoad;

export const actions = {
	delete: async ({ request, cookies }) => {
		const data = await request.formData();
		const videoIDListToDelete = data.getAll('video_delete_id_list') as string[];
		console.log('videos to delete', videoIDListToDelete);
		// TODO: Get JWT from cookies
		// TODO: Post delete request to api
	}
} satisfies Actions;
