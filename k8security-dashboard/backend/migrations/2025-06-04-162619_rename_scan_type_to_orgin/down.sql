-- This file should undo anything in `up.sql`
ALTER TABLE vulnerability
RENAME COLUMN origin TO scan_type;
