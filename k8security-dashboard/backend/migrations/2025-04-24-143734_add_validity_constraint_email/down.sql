-- This file should undo anything in `up.sql`
ALTER TABLE emails
DROP CONSTRAINT valid_email_format ;
