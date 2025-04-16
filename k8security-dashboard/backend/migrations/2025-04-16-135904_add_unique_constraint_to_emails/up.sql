-- Your SQL goes here
-- Your SQL goes here
ALTER TABLE emails
ADD CONSTRAINT unique_email
UNIQUE (email_adress);
