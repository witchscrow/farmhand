import { fetchCurrentUser } from '$lib/auth';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals, cookies }) => {
	let user = null;

	if (locals.authenticated) {
		const token = cookies.get('jwt');
		// TODO: Logout user if token is missing
		if (token) {
			user = await fetchCurrentUser(token as string);
		}
	}

	return {
		authenticated: locals.authenticated,
		user
	};
};
