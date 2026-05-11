ALTER TABLE repositories ADD COLUMN visibility TEXT NOT NULL DEFAULT 'private'
    CHECK (visibility IN ('public', 'internal', 'private'));
