-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE transaction_table
(
    id    uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    category_id uuid NOT NULL,
    name  TEXT NOT NULL,
    amount DECIMAL NOT NULL,
    notes TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
);
