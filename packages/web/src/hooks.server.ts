import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	// Get the JWT from cookies
	const jwt = event.cookies.get('jwt');
	// Set the user as authenticated in the locals object if JWT exists
	event.locals.authenticated = !!jwt;

	return await resolve(event);
};
