CREATE TYPE user_role AS ENUM ('admin', 'moderator', 'user');

CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       email TEXT NOT NULL UNIQUE,
                       hashed_password TEXT NOT NULL,
                       role user_role NOT NULL DEFAULT 'user',
                       created_at TIMESTAMPTZ DEFAULT NOW()
);
