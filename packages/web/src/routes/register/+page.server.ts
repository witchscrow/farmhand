import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { env } from '$env/dynamic/private';

export const actions = {
	default: async ({ request }) => {
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

		const response = await fetch(`${env.API_URL}/auth/register`, {
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

		if (!response.ok) {
			const errorData = await response.json();
			return fail(response.status, {
				error: errorData.message || 'Failed to create account. Please try again.',
				username: username.toString(),
				email: email.toString()
			});
		}

		// Redirect on success
		return redirect(303, '/login');
	}
} satisfies Actions;
