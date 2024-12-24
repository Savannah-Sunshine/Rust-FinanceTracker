-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE category_table
(
    id    uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name  TEXT NOT NULL,
    notes TEXT NOT NULL,
);
