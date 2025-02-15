import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(303, '/login');
	}

	return {
		user: locals.user
	};
};

export const actions = {
	updateTwitchSettings: async ({ request, fetch, locals, cookies }) => {
		if (!locals.user) {
			throw redirect(303, '/login');
		}

		const formData = await request.formData();
		const settings = {
			username: locals.user.username,
			settings: {
				stream_status_enabled: formData.get('streamStatus') === 'on',
				chat_messages_enabled: formData.get('chatMessages') === 'on',
				channel_points_enabled: formData.get('channelPoints') === 'on',
				follows_subs_enabled: formData.get('followsSubs') === 'on'
			}
		};

		try {
			const token = cookies.get('jwt');
			if (!token) {
				return fail(401, {
					success: false,
					message: 'Unauthorized'
				});
			}
			const response = await fetch(`${env.API_URL}/user/me`, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}` // Assuming token is stored in locals
				},
				body: JSON.stringify(settings)
			});

			if (!response.ok) {
				return fail(response.status, {
					success: false,
					message: 'Failed to update settings'
				});
			}

			const updatedUser = await response.json();
			return {
				success: true,
				user: updatedUser
			};
		} catch (error) {
			console.error('Error updating settings:', error);
			return fail(500, {
				success: false,
				message: 'Failed to update settings'
			});
		}
	}
} satisfies Actions;
