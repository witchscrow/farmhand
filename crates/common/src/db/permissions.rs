// The structure of a permission is role:action:resource

use super::users::UserRole;

/// A permission represents a user's ability to perform an action on a resource
/// It contains the role, action, and resource that the permission applies to
#[allow(dead_code)] // TODO: Remove this line once the permission struct is used
pub struct Permission {
    role: UserRole,
    action: Action,
    resource: Resource,
}

/// An action represents a given operation that a user can perform on a resource
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
}

impl From<String> for Action {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "create" => Action::Create,
            "read" => Action::Read,
            "update" => Action::Update,
            "delete" => Action::Delete,
            _ => {
                tracing::trace!("Invalid action: {}, defaulting to read", s);
                Action::Read // Default to read permission
            }
        }
    }
}

/// A resource represents a given entity that a user can perform actions on
pub enum Resource {
    Video,
}

impl From<String> for Resource {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "video" => Resource::Video,
            _ => {
                tracing::trace!("Invalid resource: {}, defaulting to video", s);
                Resource::Video // Default to video resource
            }
        }
    }
}

impl Permission {
    pub fn new(
        role: impl Into<UserRole>,
        action: impl Into<Action>,
        resource: impl Into<Resource>,
    ) -> Self {
        Permission {
            role: role.into(),
            action: action.into(),
            resource: resource.into(),
        }
    }
    /// Parses a role out of a string
    /// The pattern is role:action:resource
    pub fn parse_from_string(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.split(':').collect();
        if parts.len() != 3 {
            return None;
        }
        let role = UserRole::from(parts[0].to_string());
        let action = Action::from(parts[1].to_string());
        let resource = Resource::from(parts[2].to_string());
        Some(Permission::new(role, action, resource))
    }
}
