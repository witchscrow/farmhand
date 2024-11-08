import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions = {
	logout: async ({ cookies }) => {
		// Remove the JWT cookie
		cookies.delete('jwt', { path: '/' });
		// Redirect to login page
		throw redirect(303, '/login');
	}
} satisfies Actions;
