import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { env } from '$env/dynamic/private';

export const actions = {
	default: async ({ request, cookies }) => {
		const data = await request.formData();
		const username = data.get('username');
		const password = data.get('password');

		// Basic validation
		if (!username || !password) {
			return fail(400, {
				error: 'Username and password are required',
				username: username?.toString()
			});
		}

		const response = await fetch(`${env.API_URL}/auth/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username: username.toString(),
				password: password.toString()
			})
		});

		if (!response.ok) {
			const errorData = await response.json();
			return fail(response.status, {
				error: errorData.message || 'Invalid credentials',
				username: username.toString()
			});
		}
		const { token } = await response.json();

		cookies.set('jwt', token, {
			path: '/',
			expires: new Date(Date.now() + 1000 * 60 * 60 * 24), // 24 hours
			sameSite: true
		});

		throw redirect(303, '/');
	}
} satisfies Actions;
