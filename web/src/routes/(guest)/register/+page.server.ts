import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { env } from '$env/dynamic/private';

export const actions = {
	default: async ({ request, cookies }) => {
		const data = await request.formData();
		const username = data.get('username');
		const email = data.get('email');
		const password = data.get('password');
		const passwordConfirmation = data.get('passwordConfirmation');

		// Basic validation
		if (!username || !email || !password || !passwordConfirmation) {
			return fail(400, {
				error: 'All fields are required',
				username: username?.toString(),
				email: email?.toString()
			});
		}

		if (password !== passwordConfirmation) {
			return fail(400, {
				error: 'Passwords do not match',
				username: username?.toString(),
				email: email?.toString()
			});
		}
		let response = null;
		try {
			response = await fetch(`${env.API_URL}/auth/register`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					username: username.toString(),
					email: email.toString(),
					password: password.toString(),
					password_confirmation: passwordConfirmation.toString()
				})
			});
		} catch (e) {
			console.error('Error sending registration request to api', e);
			return fail(500, {
				error: 'Failed to connect to server. Please try again.',
				username: username.toString(),
				email: email.toString()
			});
		}

		if (!response.ok) {
			const res = await response.json();
			return fail(response.status, {
				error: res.message,
				username: username.toString(),
				email: email.toString()
			});
		}
		// Login the user with the token in the response
		const json = await response.json();
		// Set the cookie so we can get the user again later
		cookies.set('jwt', json.token, {
			path: '/',
			expires: new Date(Date.now() + 1000 * 60 * 60 * 24), // 24 hours
			sameSite: true
		});
		// Redirect the user
		throw redirect(303, '/');
	}
} satisfies Actions;
