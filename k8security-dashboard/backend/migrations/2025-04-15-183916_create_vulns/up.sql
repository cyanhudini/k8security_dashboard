CREATE TABLE vulnerability
(
    "id" SERIAL PRIMARY KEY,
    "vuln_id" VARCHAR NOT NULL,
    "pkg_name" VARCHAR NOT NULL,
    "pkg_id" VARCHAR NOT NULL,
    "installed_version" VARCHAR NOT NULL,
    "severity" VARCHAR NOT NULL

)