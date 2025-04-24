
CREATE TABLE emails(
    "id" SERIAL PRIMARY KEY,
    "email_adress"  VARCHAR NOT NULL,
    "receiving" BOOLEAN DEFAULT FALSE
)