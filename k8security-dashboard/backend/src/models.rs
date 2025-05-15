use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::emails)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Emails {
    pub id: i32,
    pub email_adress : String,
    pub receiving : bool,
}

#[derive(Queryable, Insertable, Serialize,Deserialize, Debug)]
#[diesel(table_name = crate::schema::emails)]
pub struct NewEmail{
    pub email_adress: String,
}


#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, Clone)]
#[diesel(table_name = crate::schema::vulnerability)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vulnerability{
    pub id: i32,
    pub vuln_id : String,
    pub pkg_name: String,
    pub pkg_id: String,
    pub installed_version: String,
    pub severity: String, 
}

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::vulnerability)]
pub struct NewVulnerability{
    pub vuln_id : String,
    pub pkg_name: String,
    pub pkg_id : String,
    pub installed_version: String,
    pub severity: String, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrivyVulnerability {
    #[serde(rename = "VulnerabilityID")]
    pub vuln_id: String,
    #[serde(rename = "PkgName")]
    pub pkg_name: String,
    #[serde(rename = "PkgID")]
    pub pkg_id: String,
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
    #[serde(rename = "Vulnerabilities")]
    pub Vulnerabilities: Option<Vec<TrivyVulnerability>>,
}

#[derive(Deserialize)]
pub struct FilterQuery {
    pub query: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct SetEmailQuery{
    pub email_id : i32,
}