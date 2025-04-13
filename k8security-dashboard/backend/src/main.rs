use backend::{create_vuln_entry, establish_connection, fetch_all_vuln_entries};

mod schema;
mod models;


fn main() {
    
    
    let connection = &mut establish_connection();
    let mut cve_id = String::from("CVE-2022-1");
    let mut pkg_name = String::new();
    let mut installed_verison = String::new();
    let mut severity = String::new();
    let vuln = create_vuln_entry(connection, cve_id, pkg_name, installed_verison, severity);
    fetch_all_vuln_entries(connection);

    
}
