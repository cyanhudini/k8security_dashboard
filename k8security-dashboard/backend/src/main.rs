use backend::{create_vuln_entry, establish_connection};

mod schema;
mod models;

fn main() {
    println!("Hello, world!");
    
    let connection = &mut establish_connection();
    let mut cve_id = String::new();
    let mut pkg_name = String::new();
    let mut installed_verison = String::new();
    let mut severity = String::new();

    let vuln = create_vuln_entry(connection, cve_id, pkg_name, installed_verison, severity);
}
