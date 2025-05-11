ALTER TABLE emails
ADD CONSTRAINT unique_email
UNIQUE (email_adress);
