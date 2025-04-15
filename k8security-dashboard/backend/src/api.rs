use std::fs::File;
use std::io::BufReader;
use actix_web::{error, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::connection;
use r2d2::{Pool, PooledConnection};
use backend::{fetch_all_vuln_entries, bulk_add_vulns};

use crate::{models::VulnerabilityReport, DbPool};


pub(crate) async fn get_all_vulns(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder>{
    let pool = pool.clone();

    let all_vulns = web::block(move || {
        let mut connection = pool.get().unwrap();

        fetch_all_vuln_entries(&mut connection)
    })
    .await?;

    Ok(web::Json(all_vulns))
}

pub(crate) async fn post_new_vulns(pool : web::Data<DbPool>) ->actix_web::Result<impl Responder>{
    let pool = pool.clone();
    web::block(move ||{
        let mut connection = pool.get().unwrap();
        bulk_add_vulns(&mut connection);
    })
    .await?;

    Ok(HttpResponse::Ok().json("Successfully inserted"))

}


pub(crate) async fn index(pool: web::Data<DbPool>) -> &'static str {
    "<p>Hello</p>"
}
