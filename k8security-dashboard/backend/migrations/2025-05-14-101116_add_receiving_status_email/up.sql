ALTER TABLE emails ADD COLUMN IF NOT EXISTS receiving BOOLEAN NOT NULL DEFAULT true;
