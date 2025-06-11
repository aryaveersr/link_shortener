-- Create 'links' Table
CREATE TABLE links (
    slug TEXT NOT NULL UNIQUE,
    href TEXT NOT NULL,
    -- uuid stored as text
    id TEXT NOT NULL,
    PRIMARY KEY (id)
);