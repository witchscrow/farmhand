import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { API_URL } from '$env/static/private';

export const load = (async () => {
	redirect(307, `${API_URL}/auth/twitch`);
}) satisfies PageServerLoad;
