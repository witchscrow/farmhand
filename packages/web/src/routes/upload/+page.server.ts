// +page.server.ts
import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';
import { redirect } from '@sveltejs/kit';

export const load = (async ({ cookies }) => {
	const token = cookies.get('jwt');
	if (!token) {
		redirect(303, '/login');
	}
	// Only pass what's needed for the client
	return {
		apiUrl: `${env.API_URL}/upload`,
		token: token
	};
}) satisfies PageServerLoad;
