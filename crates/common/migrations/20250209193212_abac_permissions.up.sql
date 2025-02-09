CREATE TYPE action_type AS ENUM ('create', 'read', 'update', 'delete');
CREATE TYPE resource_type AS ENUM ('video', 'user');

CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    action action_type NOT NULL,
    resource resource_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE permission_conditions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    permission_id UUID REFERENCES permissions(id),
    attribute_name TEXT NOT NULL, -- e.g., 'time_of_day', 'user_role', 'resource_owner'
    operator TEXT NOT NULL, -- e.g., 'equals', 'greater_than', 'contains'
    value TEXT NOT NULL, -- stored as JSON string for flexibility
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_permissions (
    user_id UUID REFERENCES users(id),
    permission_id UUID REFERENCES permissions(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, permission_id)
);

CREATE TABLE role_permissions (
    role user_role,
    permission_id UUID REFERENCES permissions(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (role, permission_id)
);
