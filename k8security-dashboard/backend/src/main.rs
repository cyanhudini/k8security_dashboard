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
    // add to Docker Container


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
