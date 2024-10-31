import { env } from '$env/dynamic/private';

export interface User {
	username: string;
	id: string;
	email: string;
}

export async function fetchCurrentUser(token: string): Promise<User | null> {
	try {
		const response = await fetch(`${env.API_URL}/user/me`, {
			headers: {
				Authorization: `Bearer ${token}`
			}
		});

		if (!response.ok) {
			throw new Error('Failed to fetch user');
		}

		return await response.json();
	} catch (error) {
		console.error('Error fetching user:', error);
		return null;
	}
}
