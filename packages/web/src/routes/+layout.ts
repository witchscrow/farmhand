import type { LayoutLoad } from './$types';
import { isAuthenticated } from '$lib/stores/auth';
import { user } from '$lib/user';

export const load: LayoutLoad = async ({ data }) => {
	isAuthenticated.set(data.authenticated);
	user.set(data.user);

	return data;
};
