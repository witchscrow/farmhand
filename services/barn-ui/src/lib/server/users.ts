import { env } from '$env/dynamic/private';
import type { User } from '$lib/stores/user';

export enum UserError {
	INVALID_TOKEN = 'INVALID_TOKEN',
	UNKNOWN = 'UNKNOWN'
}

export const getTokenIdentity = async (token: string): Promise<User | null> => {
	try {
		const headers = {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json'
		};

		const response = await fetch(`${env.API_URL}/user/me`, {
			method: 'GET',
			headers
		});

		if (response.ok) {
			const userData: User = await response.json();
			return userData;
		} else {
			console.error('Response status:', response.status);
			console.error('Response headers:', response.headers);
			const errorText = await response.text();
			console.error('Error response:', errorText);
			throw UserError.INVALID_TOKEN;
		}
	} catch (e) {
		console.error('Error in getTokenIdentity:', e);
		if (e === UserError.INVALID_TOKEN) {
			throw e;
		}
		throw UserError.UNKNOWN;
	}
};

export const getUserByEmail = async (email: string, token: string): Promise<User | null> => {
	try {
		const response = await fetch(`${env.API_URL}/user?email=${encodeURIComponent(email)}`, {
			headers: {
				Authorization: `Bearer ${token}`
			}
		});

		if (response.ok) {
			const userData: User = await response.json();
			return userData;
		} else if (response.status === 404) {
			return null;
		} else {
			throw UserError.INVALID_TOKEN;
		}
	} catch (e) {
		if (e === UserError.INVALID_TOKEN) {
			throw e;
		}
		throw UserError.UNKNOWN;
	}
};
