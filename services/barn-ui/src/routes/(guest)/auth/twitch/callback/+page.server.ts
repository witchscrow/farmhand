import { getAccessTokens, getUserInfo } from '$lib/server/twitch';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getUserByEmail } from '$lib/server/users';

export const load = (async ({ url }) => {
	const code = url.searchParams.get('code');
	const error = url.searchParams.get('error');
	const errorDescription = url.searchParams.get('error_description');

	if (error) {
		throw new Error(`Authorization failed: ${errorDescription || error}`);
	}

	if (!code) {
		throw new Error('No authorization code received from Twitch');
	}
	// Use the code to get access tokens
	const accessTokens = await getAccessTokens(code);
	// Use the tokens to get initial user data
	const twitchUser = await getUserInfo(accessTokens.access_token);
	// See if the user already exists
	const user = await getUserByEmail(twitchUser.email);
	// TODO: If user exists, update their tokens and log them in
	// TODO: If the user doesn't exist, register them
	redirect(307, '/login');
}) satisfies PageServerLoad;
