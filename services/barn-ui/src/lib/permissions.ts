// This file is for UI-related views and does not validate a user actually has a permission
// The backend is responsible for validating an action a user takes
import { UserRole } from '$lib/stores/user';

// A list of permissions and actions that a user may have
// The syntax is `RESOURCE_ACTION`
export enum Permission {
	VIDEO_DELETE,
	VIDEO_EDIT
}

// A function for returning a list of permissinos a role has access to
// These are exclusively used for client side visibility of an action
// The backend will validate the user can actually perform the action
export const getPermissionsByRole = (role: UserRole): Permission[] => {
	switch (role) {
		case UserRole.VIEWER: {
			// TODO: Implement
			return [];
		}
		case UserRole.CREATOR: {
			// TODO: Implement
			return [];
		}
		case UserRole.ADMIN: {
			return [Permission.VIDEO_DELETE, Permission.VIDEO_EDIT];
		}
		// Default case is an unauthorized or unknown role
		default: {
			return [];
		}
	}
};

// A function checking if a user has a specific permission based on their role
export const hasPermission = (role: UserRole, permission: Permission): boolean => {
	const permissions = getPermissionsByRole(role);
	return permissions.includes(permission);
};
