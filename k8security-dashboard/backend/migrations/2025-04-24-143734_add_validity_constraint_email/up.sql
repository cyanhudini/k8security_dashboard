ALTER TABLE emails
ADD CONSTRAINT valid_email_format
CHECK (email_adress ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$');