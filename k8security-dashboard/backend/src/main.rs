use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use api::*;
use diesel::PgConnection;
use std::{io, env};
use dotenv::dotenv;
mod schema;
mod models;
mod api;

type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    /* 
    TODOS: Endpoints DELETE vuln, authentication, 
    add to Docker Container
    table Welche Email ist aktiviert um updates zu erhalten und table von allen emails
    pagination der vuln entries
    components aller filter in eine Filter Komponenten packen
    da dashboard ständig läuft, sollte frontend geupdated werden vom backend ->server side update
     */

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
            .route("/vulns", web::get().to(get_all_vulns))
            .route("/add_vulns_from_file", web::get().to(post_new_vulns))
            .route("/receiver_emails", web::get().to(get_all_receiver_emails))
            .route("/add_receiver_email", web::post().to(post_new_email_adress))
            .route("/filter", web::post().to(post_filter_query))
            .route("/group_vulns_by_scan_type",  web::post().to(fetch_all_vulns_then_group))
            .route("/group_vulns_by_pkg",  web::post().to(fetch_all_vulns_then_group_by_pkg))
            .route("/set_email_status", web::post().to(update_status_email))
            .route("/delete_vulns", web::post().to(delete_vulns))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
