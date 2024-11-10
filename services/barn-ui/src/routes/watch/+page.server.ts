// +page.server.ts
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { fetchVideo } from '$lib/server/videos';

export const load = (async ({ url }) => {
	const videoID = url.searchParams.get('v');
	if (!videoID) {
		throw error(400, 'Video ID is required');
	}
	try {
		const video = await fetchVideo(videoID);
		if (video) {
			return {
				video
			};
		} else {
			throw error(404, 'Video not found');
		}
	} catch (e) {
		throw error(500, `Error fetching video ${e}`);
	}
}) satisfies PageServerLoad;
