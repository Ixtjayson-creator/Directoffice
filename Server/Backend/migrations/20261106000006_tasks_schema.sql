-- Tasks schema migration

CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    assignee UUID REFERENCES users(id) ON DELETE SET NULL,
    creator UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    priority VARCHAR(32),
    due_date TIMESTAMPTZ,
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    conversation_id UUID REFERENCES conversations(id) ON DELETE SET NULL
);

