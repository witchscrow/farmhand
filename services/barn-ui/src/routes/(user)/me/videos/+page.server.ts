import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load = (async () => {
	try {
		// TODO: Get initial table data
	} catch (e) {
		throw error(500, `Error fetching videos: ${e}`);
	}
}) satisfies PageServerLoad;
