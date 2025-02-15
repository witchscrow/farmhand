import { redirect, error } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';

export const load: PageServerLoad = async ({ url, cookies, locals }) => {
	try {
		const token = url.searchParams.get('token');
		if (token) {
			cookies.set('jwt', token, {
				path: '/',
				expires: new Date(Date.now() + 1000 * 60 * 60 * 24),
				sameSite: true,
				httpOnly: true, // Added security measure
				secure: process.env.NODE_ENV === 'production' // Secure in production
			});
		}

		return {
			loggedIn: Boolean(locals.user || token)
		};
	} catch (err) {
		console.error('Login load error:', err);
		throw error(500, 'Failed to process login request');
	}
};

export const actions = {
	default: async ({ request, cookies }) => {
		try {
			const data = await request.formData();
			const username = data.get('username');
			const password = data.get('password');

			// Validation
			if (!username || !password) {
				return {
					type: 'error',
					error: {
						message: 'Username and password are required'
					},
					data: {
						username: username?.toString()
					}
				};
			}

			// Input sanitization
			const sanitizedUsername = username.toString().trim();
			const sanitizedPassword = password.toString();

			if (!sanitizedUsername || !sanitizedPassword) {
				return {
					type: 'error',
					error: {
						message: 'Invalid input provided'
					},
					data: {
						username: sanitizedUsername
					}
				};
			}

			// API call
			let response;
			try {
				response = await fetch(`${env.API_URL}/auth/login`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({
						username: sanitizedUsername,
						password: sanitizedPassword
					})
				});
			} catch (fetchError) {
				console.error('Login API call failed:', fetchError);
				return {
					type: 'error',
					error: {
						message: 'Unable to connect to authentication service'
					},
					data: {
						username: sanitizedUsername
					}
				};
			}

			if (!response.ok) {
				const errorData = await response.json();
				return {
					type: 'error',
					error: {
						message: errorData.message || 'Invalid credentials'
					},
					data: {
						username: sanitizedUsername
					}
				};
			}

			const { token } = await response.json();

			if (!token) {
				return {
					type: 'error',
					error: {
						message: 'Invalid response from authentication service'
					},
					data: {
						username: sanitizedUsername
					}
				};
			}

			// Set cookie with security options
			cookies.set('jwt', token, {
				path: '/',
				expires: new Date(Date.now() + 1000 * 60 * 60 * 24), // 24 hours
				sameSite: true,
				httpOnly: true, // Prevents JavaScript access to the cookie
				secure: process.env.NODE_ENV === 'production' // Secure in production
			});

			throw redirect(303, '/');
		} catch (err) {
			console.error('Login action error:', err);
			return {
				type: 'error',
				error: {
					message: 'An unexpected error occurred during login'
				}
			};
		}
	}
} satisfies Actions;
