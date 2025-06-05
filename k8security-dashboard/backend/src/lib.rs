pub mod models;
pub mod schema;
use diesel::prelude::*;
use diesel::dsl::not;
use models::{NewVulnerability, Vulnerability, VulnerabilityReport, Emails, NewEmail};
use schema::emails::{email_adress, receiving, id};
use schema::vulnerability::{installed_version, pkg_name,pkg_id, severity, vuln_id, origin, scan_type};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

 #[derive(Serialize, Debug)]
pub struct GroupedVulnerabilites{
    pub vulnerabilities:HashMap<String, Vec<Vulnerability>>
}

pub fn create_vuln_entry(connection: &mut PgConnection, cve_id: String, name : String, inst_version: String, severity_grade: String, pk_id: String, ori: String, scan_type_name: String) -> Vulnerability {
    use crate::schema::vulnerability;

    let new_vuln = NewVulnerability{
        vuln_id: cve_id, pkg_name: name, installed_version: inst_version, severity: severity_grade,
        pkg_id: Some(pk_id), origin: ori, scan_type: scan_type_name,
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

pub fn filter_vuln_by_scan_type(connection: &mut PgConnection, criteria: Vec<String>) -> Vec<Vulnerability> {

    use self::schema::vulnerability::dsl::vulnerability;

    let query = vulnerability.into_boxed();
    let query = if criteria.is_empty() || criteria.iter().any(|s| s.to_uppercase() == "ALL") {
        query
    } else {
        query.filter(scan_type.eq_any(criteria))
    };
    
    query
        .load::<Vulnerability>(connection)
        .expect("Failed to load docker vulnerabilities")
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

    let new_email = NewEmail { email_adress: email_adr};

    diesel::insert_into(emails::table)
        .values(&new_email)
        .returning(Emails::as_returning())
        .get_result(connection)
        .expect("Error creating Email")
}

pub fn filter_vuln_entries_by_severity(connection: &mut PgConnection, filter_criteria : Vec<String>) -> Vec<Vulnerability>{
    use self::schema::vulnerability::dsl::vulnerability;
    let query = vulnerability.into_boxed();

    let query = if filter_criteria.is_empty() || filter_criteria.iter().any(|s| s.to_uppercase() == "ALL") {
        query
    } else {
        query.filter(severity.eq_any(filter_criteria))
    };

    query
        .load::<Vulnerability>(connection)
        .expect("Failed to load vulnerabilities")
    
}


pub fn bulk_add_vulns(connection: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::vulnerability::dsl::vulnerability;

    let file = File::open("dockerreport.json")?;
    let reader = BufReader::new(file);

    let report: VulnerabilityReport = serde_json::from_reader(reader).expect("Nicht möglich einen JSON Reader zu erstellen.");
    // if report.ClusterName is Some {
    // orign = report.ClusterName
    // else if report.ArtifactName is Some {
    // orign = report.ArtifactName
    // else {

    
    let vuln_origin = report
    .ClusterName
    .clone()
    .or_else(|| report.ArtifactName.clone())
    .unwrap_or_else(|| "unknown".to_string());


    let mut new_vulns: Vec<NewVulnerability> = Vec::new();
    if let Some(resources) = &report.Resources {
        for resource in resources {
            if let Some(results) = &resource.Results {
                for result in results {
                    if let Some(v) = &result.Vulnerabilities {
                        for v in v {
                            new_vulns.push(NewVulnerability {
                                vuln_id: v.vuln_id.clone(),
                                pkg_name: v.pkg_name.clone(),
                                pkg_id: Some(v.pkg_id.clone().unwrap_or_default()),
                                installed_version: v.installed_version.clone(),
                                severity: v.severity.clone(),
                                origin: vuln_origin.clone(),
                                scan_type: "k8s".to_string(),

                            });
                        }
                    }
                }
            }
        }
    } else if let Some(results) = report.Results {
        for result in results {
            if let Some(vulns) = result.Vulnerabilities {
                for v in vulns {
                    new_vulns.push(NewVulnerability {
                        vuln_id: v.vuln_id,
                        pkg_name: v.pkg_name,
                        pkg_id: Some(v.pkg_id.clone().unwrap_or_default()),
                        installed_version: v.installed_version,
                        severity: v.severity,
                        origin: vuln_origin.clone(), 
                        scan_type: "docker".to_string(),
                    });
                }
            }
        }
    }
   
    diesel::insert_into(vulnerability)
        .values(&new_vulns)
        .on_conflict((vuln_id, pkg_name, pkg_id, installed_version))
        .do_nothing()
        .execute(connection)
        .expect("Error inserting new vulnerabilities");

    Ok(())

}

pub fn get_grouped_by_docker_scan_type(connection: &mut PgConnection, filter : Vec<String>) -> GroupedVulnerabilites  {
    // if entry has scan_type docker, it then should be further grouped by ArtifactName
    let to_be_grouped = filter_vuln_by_scan_type(connection, vec!["docker".to_string()]);
    print!(" grouped {:?}", to_be_grouped);
    let mut grouped: HashMap<String, Vec<Vulnerability>> = HashMap::new();
    for vuln in to_be_grouped {
        let key = format!("{}", vuln.origin);
        grouped.entry(key).or_insert(vec![]).push(vuln);
    }
    let g = GroupedVulnerabilites {
        vulnerabilities: grouped,
    };
    
    println!("{:?}", g);
    g
}


// TODO führe group_by_pkgid_pkgname und get_grouped_by_docker_scan_type zusammen
pub fn group_by_pkgid_pkgname(connection: &mut PgConnection) -> GroupedVulnerabilites {

   let to_be_grouped = fetch_all_vuln_entries(connection);

   let mut grouped : HashMap<String, Vec<Vulnerability>> = HashMap::new();

    for vuln in to_be_grouped {
        let key = format!("{}|{}", vuln.pkg_id, vuln.pkg_name);
        grouped.entry(key).or_insert(vec![]).push(vuln);
    }
    let mut g = GroupedVulnerabilites{
        vulnerabilities: grouped,
    };
    //TODO: nochmal gucken was ich mir dabei gedacht habe
    let f = filter_grouped_by_severity(&mut g);
    let mut g_filtered = GroupedVulnerabilites{
        vulnerabilities: f,
    };
    g_filtered
}

pub fn filter_grouped_by_severity(groupedVulns : &mut GroupedVulnerabilites) -> HashMap<String, Vec<Vulnerability>>{
    let f_crit = vec!["CRITICAL.".to_string()];
    
    let f: HashMap<_, _> = groupedVulns.vulnerabilities
    .iter()
    .filter_map(|(k, vulns)| {
        let filtered_vulns: Vec<Vulnerability> = vulns.iter().filter(|v| v.severity == "CRITICAL".to_string()).cloned().collect();
        if filtered_vulns.is_empty() {
            None
        } else {
            Some((k.clone(), filtered_vulns))
        }
    })
    .collect();
    f
}

pub fn update_email_entry(connection: &mut PgConnection, email_id: i32) -> Emails{
    use self::schema::emails::dsl::emails;

    diesel::update(emails.filter(id.eq(email_id)))
        .set(receiving.eq(not(receiving)))
        .get_result(connection)
        .expect("Error updating email status")
}