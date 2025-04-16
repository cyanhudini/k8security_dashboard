pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use models::{NewVulnerability, Vulnerability, VulnerabilityReport, Emails};
use schema::vulnerability::{installed_version, pkg_name, severity, vuln_id};
use schema::emails::email_address;
use std::env;
use std::fs::File;
use std::io::BufReader;

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

pub fn update_vuln_entry(connection: &mut PgConnection, cve_id: Option<String>, name : Option<String>, inst_version: Option<String>, severity_grade: Option<String> ){

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

pub fn filter_vuln_entries_by_severity(connection: &mut PgConnection){
    use self::schema::vulnerability::dsl::vulnerability;
    let vulns = vulnerability
        .filter(severity.eq_any(["HIGH"]))
        .load::<Vulnerability>(connection)
        .expect("Etwas ist schiefgelaufen.");

    for vuln in vulns {
        println!("{:?} {:?} {:?} {:?}", vuln.vuln_id, vuln.installed_version, vuln.pkg_name, vuln.id);
    }  
    
}
// check for malformed json

pub fn process_trivy_report_to_db_json(){
    /*
    extra json oder nur als Struct 
    */

}

pub fn filter_out_duplicates_from_json(){
}

pub  fn bulk_add_vulns(connection: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::vulnerability::dsl::vulnerability;

    print!("DDD");
    let file = File::open("report.json")?;
    let reader = BufReader::new(file);

    let report: VulnerabilityReport = serde_json::from_reader(reader).expect("msg");
    print!("add in bulk");

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
