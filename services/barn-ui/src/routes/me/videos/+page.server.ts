import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { fetchVideos } from '$lib/server/videos';

export const load = (async ({ locals }) => {
	try {
		if (!locals.user) {
			throw redirect(401, '/login');
		}
		const videos = await fetchVideos({
			channel: locals.user.username
		});
		if (videos) {
			return {
				videos
			};
		} else {
			throw error(404, 'No videos found');
		}
	} catch (e) {
		throw error(500, `Error fetching videos: ${e}`);
	}
}) satisfies PageServerLoad;
