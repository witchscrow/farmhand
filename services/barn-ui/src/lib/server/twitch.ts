import { env } from '$env/dynamic/private';

// TwitchCredentials for interacting with Twitch's API
type TwitchCredentials = {
	id: string;
	secret: string;
	redirectURI: string;
	scope: string; // A space-delimited list of scopes
};

// Enabled twitch scopes for application to function
// NOTE: Maybe make this configurable in the future
const enabledScopes = ['channel:bot', 'user:read:email', 'user:read:chat'];

// Parses the enabled scopes into the appropriate format for twitch
// A space-delimited list of scopes
const getTwitchScopes = (): string => {
	return enabledScopes.join(' ');
};

// Get Twitch Credentials from environment
// Throws an error if any of the variables are missing
export const getTwitchCredentials = (): TwitchCredentials => {
	const id = env.TWITCH_CLIENT_ID;
	const secret = env.TWITCH_CLIENT_SECRET;
	const redirectURI = env.TWITCH_REDIRECT_URI;
	const scope = getTwitchScopes();
	if (!id || !secret || !redirectURI || !scope) {
		throw new Error('Missing required Twitch credentials');
	}

	return {
		id,
		secret,
		redirectURI,
		scope
	};
};

// Base URL for interacting with Twitch OAuth
const BASE_OAUTH_URL = 'https://id.twitch.tv/oauth2/authorize';

// Generates an OAuth url to hand to a user for authorizing the application
export const generateOAuthURL = () => {
	const creds = getTwitchCredentials();
	const params = new URLSearchParams({
		response_type: 'code',
		client_id: creds.id,
		redirect_uri: creds.redirectURI,
		scope: creds.scope
	});

	return `${BASE_OAUTH_URL}?${params.toString()}`;
};

// Twitch tokens expected when exchanging an authorization code
type TwitchAccessTokens = {
	access_token: string;
	expires_in: number;
	refresh_token: string;
	scope: string[];
	token_type: string;
};

// Gets access tokens using provided authorization code
export const getAccessTokens = async (code: string): Promise<TwitchAccessTokens> => {
	const creds = getTwitchCredentials();
	const params = new URLSearchParams({
		client_id: creds.id,
		client_secret: creds.secret,
		code: code,
		grant_type: 'authorization_code',
		redirect_uri: creds.redirectURI
	});

	const response = await fetch('https://id.twitch.tv/oauth2/token', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/x-www-form-urlencoded'
		},
		body: params.toString()
	});

	if (!response.ok) {
		throw new Error(`Bad response from Twitch getting tokens, status: ${response.status}`);
	}

	const data = (await response.json()) as TwitchAccessTokens;
	if (!data.access_token || !data.refresh_token) {
		throw new Error('Invalid token response from Twitch');
	}

	return data;
};

// Refresh access tokens using refresh token
export const refreshAccessTokens = async (refreshToken: string) => {
	const creds = getTwitchCredentials();
	const params = new URLSearchParams({
		grant_type: 'refresh_token',
		refresh_token: refreshToken,
		client_id: creds.id,
		client_secret: creds.secret
	});

	const response = await fetch('https://id.twitch.tv/oauth2/token', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/x-www-form-urlencoded'
		},
		body: params.toString()
	});

	if (!response.ok) {
		throw new Error(`Bad response from Twitch refreshing tokens, status: ${response.status}`);
	}
	const data: TwitchAccessTokens = await response.json();
	return data;
};

// User information returned from Twitch
type TwitchUserInfo = {
	id: string;
	login: string;
	display_name: string;
	type: string;
	broadcaster_type: string;
	description: string;
	profile_image_url: string;
	offline_image_url: string;
	view_count: number;
	email: string;
	created_at: string;
};

// Get user information from Twitch using access token
export const getUserInfo = async (accessToken: string): Promise<TwitchUserInfo> => {
	const creds = getTwitchCredentials();
	const response = await fetch('https://api.twitch.tv/helix/users', {
		headers: {
			Authorization: `Bearer ${accessToken}`,
			'Client-Id': creds.id
		}
	});

	if (!response.ok) {
		throw new Error(`Failed to fetch user info from Twitch, status: ${response.status}`);
	}

	const data = await response.json();
	if (!data.data?.[0]) {
		throw new Error('Invalid user info response from Twitch');
	}

	return data.data[0] as TwitchUserInfo;
};
