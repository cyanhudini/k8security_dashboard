pub mod models;
pub mod schema;
use diesel::dsl::not;
use diesel::prelude::*;
use models::{Emails, NewEmail, NewVulnerability, Vulnerability, VulnerabilityReport};
use schema::emails::{email_adress, id, receiving};
use schema::vulnerability::{
    installed_version, origin, pkg_id, pkg_name, scan_type, severity, vuln_id,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Debug)]
pub struct GroupedVulnerabilites {
    pub vulnerabilities: HashMap<String, Vec<Vulnerability>>,
}

pub fn create_vuln_entry(
    connection: &mut PgConnection,
    cve_id: String,
    name: String,
    inst_version: String,
    severity_grade: String,
    pk_id: String,
    ori: String,
    scan_type_name: String,
) -> Vulnerability {
    use crate::schema::vulnerability;

    let new_vuln = NewVulnerability {
        vuln_id: cve_id,
        pkg_name: name,
        installed_version: inst_version,
        severity: severity_grade,
        pkg_id: Some(pk_id),
        origin: ori,
        scan_type: scan_type_name,
    };

    diesel::insert_into(vulnerability::table)
        .values(&new_vuln)
        .returning(Vulnerability::as_returning())
        .get_result(connection)
        .expect("Error creating new Vulnerability")
}

pub fn fetch_all_vuln_entries(connection: &mut PgConnection) -> Vec<Vulnerability> {
    use self::schema::vulnerability::dsl::vulnerability;
    vulnerability.load::<Vulnerability>(connection).unwrap()
}

pub fn fetch_all_vuln_filtered_scan_type(connection: &mut PgConnection,criteria: Vec<String>) -> Vec<Vulnerability> {
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

pub fn delete_vuln_entry(connection: &mut PgConnection, to_delete: Vec<i32>) {
    use self::schema::vulnerability::dsl::vulnerability;
    println!("Deleting vulnerabilities with IDs: ");
    //delete vulnerabilities based on the provided IDs
    diesel::delete(vulnerability)
        .filter(schema::vulnerability::id.eq_any(to_delete))
        .execute(connection)
        .expect("Unable to delete");
}

pub fn create_email_entry(connection: &mut PgConnection, email_adr: String) -> Emails {
    use crate::schema::emails;

    let new_email = NewEmail {
        email_adress: email_adr,
    };

    diesel::insert_into(emails::table)
        .values(&new_email)
        .returning(Emails::as_returning())
        .get_result(connection)
        .expect("Error creating Email")
}

pub fn filter_vuln_entries_by_severity(connection: &mut PgConnection,filter_criteria: Vec<String>,) -> Vec<Vulnerability> {
    use self::schema::vulnerability::dsl::vulnerability;
    let query = vulnerability.into_boxed();

    let query = if filter_criteria.is_empty()
        || filter_criteria.iter().any(|s| s.to_uppercase() == "ALL")
    {
        query
    } else {
        query.filter(severity.eq_any(filter_criteria))
    };

    query
        .load::<Vulnerability>(connection)
        .expect("Failed to load vulnerabilities")
}

pub fn add_vulns_from_file(
    connection: &mut PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::vulnerability::dsl::vulnerability;

    let file = File::open("/app/report.json")?;
    let reader = BufReader::new(file);

    let report: VulnerabilityReport =
        serde_json::from_reader(reader).expect("Nicht möglich einen JSON Reader zu erstellen.");

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

pub fn group_by_docker_scan_type(connection: &mut PgConnection, filter: Vec<String>) -> GroupedVulnerabilites {

    let to_be_grouped = fetch_all_vuln_filtered_scan_type(connection, filter);
    /*
    let mut grouped: HashMap<String, Vec<Vulnerability>> = HashMap::new();
    for vuln in to_be_grouped {
        let key = format!("{}", vuln.origin);
        grouped.entry(key).or_insert(vec![]).push(vuln);
    }
    */
    let grouped = group_vulnerabilites_by_criteria(to_be_grouped, |v| v.origin.clone());

    let g = GroupedVulnerabilites {
        vulnerabilities: grouped,
    };
    
    g
}

// TODO führe group_by_pkgid_pkgname und group_by_docker_scan_type zusammen
pub fn group_by_pkgid_pkgname(connection: &mut PgConnection, filter: Vec<String>) -> GroupedVulnerabilites {
    let to_be_grouped = fetch_all_vuln_entries(connection);
   
    let g = group_vulnerabilites_by_criteria(to_be_grouped, |v| {
        format!("{}|{}", v.pkg_id, v.pkg_name)
    });
   

    let mut grouped_vulns = GroupedVulnerabilites { vulnerabilities: g };
    let grouped_vulns_then_filtered = filter_grouped2_by_severity(&mut grouped_vulns, filter);
    //print!("Grouped and filtered: {:?}", g_filtered);
    GroupedVulnerabilites { vulnerabilities: grouped_vulns_then_filtered }
}

pub fn filter_grouped_by_severity(groupedVulns: &mut GroupedVulnerabilites) -> HashMap<String, Vec<Vulnerability>> {
    // TODO: filter muss natürlich noch für alle funktionieren nicht nur CRITICAL
    let f: HashMap<_, _> = groupedVulns
        .vulnerabilities
        .iter()
        .filter_map(|(k, vulns)| {
            let filtered_vulns: Vec<Vulnerability> = vulns
                .iter()
                .filter(|v| v.severity == "CRITICAL".to_string())
                .cloned()
                .collect();
            if filtered_vulns.is_empty() {
                None
            } else {
                Some((k.clone(), filtered_vulns))
            }
        })
        .collect();
    f
}


pub fn filter_grouped2_by_severity(
    grouped_vulns: &GroupedVulnerabilites,
    filter_criteria: Vec<String>,
) -> HashMap<String, Vec<Vulnerability>> {
    let severity_set: HashSet<_> = filter_criteria.into_iter().collect();
    //HashMap eignet sich prima zum Gruppieren, x -> many y's
    grouped_vulns
        .vulnerabilities
        .iter()
        .filter_map(|(k, vulns)| {
            
            let is_severity_set = severity_set.is_empty() || severity_set.contains(&"ALL".to_string());
            let filtered = if is_severity_set {
                //print!("{:?}", vulns);
                vulns.clone()
            } else {
                vulns
                    .iter()
                    .filter(|v| severity_set.contains(&v.severity.to_uppercase()))
                    .cloned()
                    .collect()
            };

            if filtered.is_empty() {
                None
            } else {
                Some((k.clone(), filtered))
            }
        })
        .collect()
}


pub fn update_email_entry(connection: &mut PgConnection, email_id: i32) -> Emails {
    use self::schema::emails::dsl::emails;

    diesel::update(emails.filter(id.eq(email_id)))
        .set(receiving.eq(not(receiving)))
        .get_result(connection)
        .expect("Error updating email status")
}

pub fn group_vulnerabilites_by_criteria<F, K>(
    vulnerabilities: Vec<Vulnerability>,
    criteria_fn: F,
) -> HashMap<K, Vec<Vulnerability>>
where
    F: Fn(&Vulnerability) -> K,
    K: Eq + Hash,
{
    let mut grouped_vulns: HashMap<K, Vec<Vulnerability>> = HashMap::new();
    for vuln in vulnerabilities {
        let key = criteria_fn(&vuln);
        grouped_vulns.entry(key).or_insert_with(Vec::new).push(vuln);
    }
    grouped_vulns
}

#[cfg(test)]
mod tests {
    

    use super::*;
    use crate::models::Vulnerability;

    //Helferfunktion zum Erstellen von Mockdaten
    fn create_test_vuln(id_string: i32, severity_string: &str, origin_string: &str, pkg_name_string: &str) -> Vulnerability {
        Vulnerability {
            id: id_string,
            vuln_id: "CVE-2024-TEST".to_string(),
            pkg_name: pkg_name_string.to_string(),
            pkg_id: "pkg-id".to_string(),
            installed_version: "1.0.0".to_string(),
            severity: severity_string.to_string(),
            origin: origin_string.to_string(),
            scan_type: "docker".to_string(),
        }
    }

    #[test]
    fn test_filter_grouped2_by_severity_critical(){
        let mut grouped_vulns = HashMap::new();
        grouped_vulns.insert(
            "test_group".to_string(),
            vec![
                create_test_vuln(1, "CRITICAL", "k8s", "pkg-Test-CRITICAL"),
                create_test_vuln(2, "HIGH", "k8s", "pkg-Test-HIGH"),
                create_test_vuln(3, "LOW", "k8s", "pkg")
                ],
            
        );
        let grouped_vulns_struct = GroupedVulnerabilites {
            vulnerabilities: grouped_vulns,
        };
        let filter = vec!["CRITICAL".to_string()];
        let result = filter_grouped2_by_severity(&grouped_vulns_struct, filter);
        // Erwwartung: Nur eine Vuln ist enthalten
        assert_eq!(result.len(), 1);
        assert!(result.get("test_group").unwrap()[0].pkg_name == "pkg-Test-CRITICAL");
        
    }

    
    #[test]
    fn test_generalized_group_vulnerabilites(){
        let to_be_grouped = vec![
                create_test_vuln(1, "CRITICAL", "k8s", "pkg-Test-CRITICAL"),
                create_test_vuln(2, "HIGH", "k8s", "pkg-Test-HIGH"),
                create_test_vuln(3, "LOW", "k8s", "pkg")
                ];

        let grouped = group_vulnerabilites_by_criteria(to_be_grouped, |v| v.origin.clone());
        // Zwei Gruppen, da jede vuln k8s als origin hat
        assert_eq!(grouped.len(), 1);      
        
    }
}