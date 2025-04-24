-- This file should undo anything in `up.sql`
ALTER TABLE emails
ALTER COLUMN receiving SET DEFAULT FALSE;

