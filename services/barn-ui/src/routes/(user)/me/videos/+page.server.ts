import type { PageServerLoad } from './$types';
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
