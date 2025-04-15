use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use api::{get_all_vulns, index, post_new_vulns};
use backend::{create_vuln_entry, establish_connection, fetch_all_vuln_entries, filter_vuln_entries_by_severity};
use diesel::PgConnection;
use std::{io, env};
use dotenv::dotenv;
mod schema;
mod models;
mod api;

type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Endpoints GET 1, GET all, FILTER POST vuln, DELETE vuln,
    // add to Docker Container

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL muss zu einer PostgreSQL Datenbank zeigen");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/vulns", web::get().to(get_all_vulns))
            .route("/add_vulns_bulk", web::get().to(post_new_vulns))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
