import { writable } from 'svelte/store';

export interface User {
	username: string;
	email: string;
}

export const user = writable<User | null>(null);
