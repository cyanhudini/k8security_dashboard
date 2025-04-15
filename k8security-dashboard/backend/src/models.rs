use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::vulnerability)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vulnerability{
    pub id: i32,
    pub vuln_id : String,
    pub pkg_name: String,
    pub installed_version: String,
    pub severity: String, 
}

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::vulnerability)]
pub struct NewVulnerability{
    pub vuln_id : String,
    pub pkg_name: String,
    pub installed_version: String,
    pub severity: String, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrivyVulnerability {
    #[serde(rename = "VulnerabilityID")]
    pub vuln_id: String,
    #[serde(rename = "PkgName")]
    pub pkg_name: String,
    #[serde(rename = "InstalledVersion")]
    pub installed_version: String,
    #[serde(rename = "Severity")]
    pub severity: String,
}
#[derive(Deserialize)]
pub struct VulnerabilityReport {
    pub Resources: Vec<Resource>,
}

#[derive(Deserialize)]
pub struct Resource {
    pub Results: Vec<ResultEntry>,
}

#[derive(Deserialize)]
pub struct ResultEntry {
    pub Vulnerabilities: Option<Vec<TrivyVulnerability>>,
    // Add other fields as needed
}

