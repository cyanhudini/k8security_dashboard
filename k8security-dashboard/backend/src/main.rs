use actix_web::{web, App, HttpServer};
use api::get_all_vulns;
use backend::{create_vuln_entry, establish_connection, fetch_all_vuln_entries, filter_vuln_entries_by_severity};
use diesel::PgConnection;
use std::{io, env};
mod schema;
mod models;
mod api;

type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // TODO: while Loop, serialize into json
    // Endpoints GET 1, GET all, FILTER POST vuln, DELETE vuln,
    // 

    
    let connection = &mut establish_connection();
    let mut cve_id = String::from("CVE-2022-1");
    let mut pkg_name = String::from("R");
    let mut installed_verison = String::from("1.0.0");
    let mut severity = String::from("HIGH");
    let vuln = create_vuln_entry(connection, cve_id, pkg_name, installed_verison, severity);
    fetch_all_vuln_entries(connection);
    filter_vuln_entries_by_severity(connection);
    println!("Added to Database");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL muss zu einer PostgreSQL Datenbank zeigen");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_all_vulns)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
