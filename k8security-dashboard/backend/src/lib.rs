pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::dsl::not;
use models::{NewVulnerability, Vulnerability, VulnerabilityReport, Emails, NewEmail};
use schema::emails::{email_adress, receiving};
use schema::vulnerability::{installed_version, pkg_name, severity, vuln_id};
use std::fs::File;
use std::io::BufReader;


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

pub fn fetch_all_vuln_entries(connection: &mut PgConnection) -> Vec<Vulnerability>{
    use self::schema::vulnerability::dsl::vulnerability; 
    vulnerability.load::<Vulnerability>(connection).unwrap()
}

pub fn fetch_receiver_emails(connection: &mut PgConnection) -> Vec<Emails> {
    use self::schema::emails::dsl::emails;
    emails.load::<Emails>(connection).unwrap()
}

pub fn delete_vuln_entry(connection: &mut PgConnection, to_delete : Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    use self::schema::vulnerability::dsl::vulnerability;
    diesel::delete(vulnerability.filter(vuln_id
        .eq_any(to_delete)))
        .execute(connection)
        .expect("Unable to delete");
    Ok(())
}

pub fn create_email_entry(connection: &mut PgConnection, email_adr : String) -> Emails {
    use crate::schema::emails;

    let new_email = NewEmail { email_adress: email_adr, receiving: true};

    diesel::insert_into(emails::table)
        .values(&new_email)
        .returning(Emails::as_returning())
        .get_result(connection)
        .expect("Error creating Email")
}

pub fn filter_vuln_entries_by_severity(connection: &mut PgConnection, filter_criteria : Vec<String>) -> Vec<Vulnerability>{
    use self::schema::vulnerability::dsl::vulnerability;
    let query = vulnerability.into_boxed();

    let query = if filter_criteria.is_empty() || 
               filter_criteria.iter().any(|s| s.to_uppercase() == "ALL") {
        query
    } else {
        query.filter(severity.eq_any(filter_criteria))
    };

    query
        .load::<Vulnerability>(connection)
        .expect("Failed to load vulnerabilities")
    
}

pub  fn bulk_add_vulns(connection: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::vulnerability::dsl::vulnerability;

    let file = File::open("report.json")?;
    let reader = BufReader::new(file);

    let report: VulnerabilityReport = serde_json::from_reader(reader).expect("Nicht möglich einen JSON Reader zu erstellen.");

    let mut new_vulns: Vec<NewVulnerability> = Vec::new();
    for resource in report.Resources {
        for result in resource.Results {
            if let Some(vulns) = result.Vulnerabilities {
                for v in vulns {
                    new_vulns.push(NewVulnerability {
                        vuln_id: v.vuln_id,
                        pkg_name: v.pkg_name,
                        installed_version: v.installed_version,
                        severity: v.severity,
                    });
                }
            }
        }
    }

    diesel::insert_into(vulnerability)
        .values(&new_vulns)
        .on_conflict((vuln_id, pkg_name, installed_version))
        .do_nothing()
        .execute(connection)
        .expect("Error inserting new vulnerabilities");

    Ok(())


}


pub fn update_email_entry(connection: &mut PgConnection, query_email: String) -> Emails{
    use self::schema::emails::dsl::emails;
    diesel::update(emails.filter(email_adress.eq(query_email)))
        .set(receiving.eq(not(receiving)))
        .get_result(connection)
        .expect("Error updating email status")
}