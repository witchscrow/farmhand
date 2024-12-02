import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';

export const load = (async () => {
	redirect(307, `${env.API_URL}/auth/twitch`);
}) satisfies PageServerLoad;
