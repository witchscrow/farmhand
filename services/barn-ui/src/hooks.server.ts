import { getTokenIdentity, UserError } from '$lib/server/users';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	// Get the JWT from cookies
	const jwt = event.cookies.get('jwt');
	// If a jwt exists and there is no user, get the user
	if (jwt && !event.locals.user) {
		try {
			const user = await getTokenIdentity(jwt);
			if (user) {
				event.locals.user = user;
			}
		} catch (e) {
			if (e === UserError.INVALID_TOKEN) {
				console.error('Invalid JWT token');
			} else {
				console.error('Unknown error getting user');
			}
			// If there was an error getting the user, delete the jwt cookie
			event.cookies.delete('jwt', { path: '/' });
		}
	}

	return await resolve(event);
};
