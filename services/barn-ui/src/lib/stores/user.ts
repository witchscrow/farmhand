import { writable } from 'svelte/store';

export enum UserRole {
	VIEWER = 'Viewer',
	CREATOR = 'Creator',
	ADMIN = 'Admin'
}

export interface User {
	username: string;
	email: string;
	role: UserRole;
}

export const user = writable<User | null>(null);
