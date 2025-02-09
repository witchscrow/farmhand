//! Permission module
//!
//! This module defines the `Permission` struct and associated enums for actions and resources.
//! It also provides methods for creating and parsing permissions.

use uuid::Uuid;

use super::User;

/// A permission represents a user's ability to perform an action on a resource
/// It contains the role, action, and resource that the permission applies to
#[allow(dead_code)] // TODO: Remove this line once the permission struct is used
pub struct Permission {
    action: Action,
    resource: Resource,
}

/// Context for evaluating permissions with additional attributes
#[derive(Debug)]
pub struct PermissionContext {
    pub user: User,
    pub resource: Resource,
    pub action: Action,
    pub resource_owner_id: Option<Uuid>,
}

impl PermissionContext {}

/// Represents a condition that must be met for a permission to be granted
pub struct PermissionCondition {
    pub attribute_name: String,
    pub operator: String,
    pub value: String,
}

impl PermissionCondition {
    pub fn evaluate(&self, context: &PermissionContext) -> bool {
        match self.attribute_name.as_str() {
            "resource_owner" => self.evaluate_ownership(context),
        }
    }

    fn evaluate_ownership(&self, context: &PermissionContext) -> bool {
        match self.operator.as_str() {
            "equals" => context.resource_owner_id == Some(context.user.id),
            "!=" => context.resource_owner_id != Some(context.user.id),
            _ => panic!("Invalid operator: {}", self.operator),
        }
    }
}

/// An action represents a given operation that a user can perform on a resource
#[derive(Debug)]
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
            _ => panic!("Invalid action: {}", s),
        }
    }
}

/// A resource represents a given entity that a user can perform actions on
#[derive(Debug)]
pub enum Resource {
    Video,
    User,
}

impl From<String> for Resource {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "video" => Resource::Video,
            "user" => Resource::User,
            _ => panic!("Invalid resource: {}", s),
        }
    }
}

impl Permission {
    pub fn new(action: impl Into<Action>, resource: impl Into<Resource>) -> Self {
        Permission {
            action: action.into(),
            resource: resource.into(),
        }
    }
    /// Parses a role out of a string
    /// The pattern is action:resource
    pub fn parse_from_string(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let action = Action::from(parts[0].to_string());
        let resource = Resource::from(parts[1].to_string());
        Some(Permission::new(action, resource))
    }
}
