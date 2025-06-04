ALTER TABLE vulnerability
DROP CONSTRAINT IF EXISTS unique_vuln;

ALTER TABLE vulnerability
ADD CONSTRAINT unique_vuln UNIQUE (vuln_id, pkg_name, pkg_id, installed_version);
