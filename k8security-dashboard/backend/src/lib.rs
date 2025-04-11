pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use models::{NewVulnerability, Vulnerability};
use schema::vulnerability::{pkg_name, vuln_id};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_vuln_entry(connection: &mut PgConnection, cve_id: String, name : String, inst_version: String, severity_grade: String)-> Vulnerability{
    use crate::schema::vulnerability;

    let new_vuln = NewVulnerability{
        vuln_id: cve_id, pkg_name: name, installed_version: inst_version, severity: severity_grade
    };

    diesel::insert_into(vulnerability::table)
        .values(&new_vuln)
        .returning(Vulnerability::as_returning())
        .get_result(connection)
        .expect("Error creating new Vulnerability")
}