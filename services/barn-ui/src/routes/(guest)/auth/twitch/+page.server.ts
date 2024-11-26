import { generateOAuthURL } from '$lib/server/twitch';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = (async () => {
	const twitchURL = generateOAuthURL();
	redirect(307, twitchURL);
}) satisfies PageServerLoad;
