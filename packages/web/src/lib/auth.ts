import { isAuthenticated } from './stores/auth';
import { goto } from '$app/navigation';
import { env } from '$env/dynamic/private';

interface AuthResponse {
	ok: boolean;
	error?: string;
}

export async function login(username: string, password: string): Promise<AuthResponse> {
	try {
		const response = await fetch(`${env.API_URL}/auth/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ username, password })
		});

		if (response.ok) {
			isAuthenticated.set(true);
			return { ok: true };
		}

		return {
			ok: false,
			error: 'Invalid credentials'
		};
	} catch (error) {
		console.error('Could not login', error);
		return {
			ok: false,
			error: 'An error occurred during login'
		};
	}
}

export async function register(
	username: string,
	email: string,
	password: string,
	passwordConfirmation: string
): Promise<AuthResponse> {
	try {
		const response = await fetch(`${env.API_URL}/auth/register`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username,
				email,
				password,
				password_confirmation: passwordConfirmation
			})
		});

		if (response.ok) {
			isAuthenticated.set(true);
			return { ok: true };
		}

		return {
			ok: false,
			error: 'Registration failed'
		};
	} catch (error) {
		console.error('Could not register', error);
		return {
			ok: false,
			error: 'An error occurred during registration'
		};
	}
}

export async function logout() {
	try {
		await fetch(`${env.API_URL}/auth/logout`, {
			method: 'POST'
		});
		isAuthenticated.set(false);
		goto('/login');
	} catch (error) {
		console.error('Logout failed:', error);
	}
}
