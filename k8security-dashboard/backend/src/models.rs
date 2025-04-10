use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vulnerability)]
pub struct Vulnerability{
    pub id: i32,
    pub vuln_id : i32,
    pub pkg_name: String,
    pub installed_version: String,
    pub severity: String, 
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::vulnerability)]
pub struct NewVulnerability{
    pub vuln_id : i32,
    pub pkg_name: String,
    pub installed_version: String,
    pub severity: String, 
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::vulnerability)]
pub struct UpdateVulnerability{
    pub vuln_id : Option<i32>,
    pub pkg_name: Option<String>,
    pub installed_version: Option<String>,
    pub severity: Option<String>, 
}