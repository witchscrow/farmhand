import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
	const tmpUser = {
		name: 'sneakycrow'
	};
	const user: { name: string } | null = locals.authenticated ? tmpUser : null;
	return {
		authenticated: locals.authenticated,
		user
	};
};
