-- This file should undo anything in `up.sql`
ALTER TABLE emails
DROP CONSTRAINT unique_email;
