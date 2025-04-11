use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vulnerability)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vulnerability{
    pub id: i32,
    pub vuln_id : String,
    pub pkg_name: String,
    pub installed_version: String,
    pub severity: String, 
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::vulnerability)]
pub struct NewVulnerability{
    pub vuln_id : String,
    pub pkg_name: String,
    pub installed_version: String,
    pub severity: String, 
}
