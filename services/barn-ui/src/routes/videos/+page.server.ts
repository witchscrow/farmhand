import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { fetchVideos } from '$lib/server/videos';

export const load = (async () => {
	try {
		const videos = await fetchVideos();
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
