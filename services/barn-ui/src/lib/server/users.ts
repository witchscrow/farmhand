import { env } from '$env/dynamic/private';
import type { User } from '$lib/stores/user';

export enum UserError {
	INVALID_TOKEN = 'INVALID_TOKEN',
	UNKNOWN = 'UNKNOWN'
}

export const getTokenIdentity = async (token: string): Promise<User | null> => {
	try {
		console.log('Sending request with token:', token); // Debug log

		const headers = {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json',
			Origin: 'https://staging.farmhand.witchscrow.com'
		};

		console.log('Request headers:', headers); // Debug log

		const response = await fetch(`${env.API_URL}/user/me`, {
			method: 'GET',
			headers,
			credentials: 'include',
			mode: 'cors'
		});

		console.log('Response status:', response.status); // Debug log
		console.log('Response headers:', Object.fromEntries(response.headers)); // Debug log

		if (response.ok) {
			const userData: User = await response.json();
			return userData;
		} else {
			console.error('Response not OK:', await response.text()); // Debug log
			throw UserError.INVALID_TOKEN;
		}
	} catch (e) {
		console.error('Error in getTokenIdentity:', e); // Debug log
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
