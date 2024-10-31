import { fetchCurrentUser } from '$lib/auth';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals, cookies }) => {
	let user = null;

	if (locals.authenticated) {
		const token = cookies.get('jwt');
		if (token) {
			user = await fetchCurrentUser(token as string);
			// TODO: Logout user
		}
	}

	return {
		authenticated: locals.authenticated,
		user
	};
};
