use actix_web::{error, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use r2d2::{Pool, PooledConnection};
use backend::fetch_all_vuln_entries;

use crate::DbPool;


pub(crate) async fn get_all_vulns(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder>{
    let pool = pool.clone();

    let all_vulns = web::block(move || {
        let mut connection = pool.get().unwrap();

        fetch_all_vuln_entries(&mut connection)
    })
    .await?;

    Ok(web::Json(all_vulns))
}

pub(crate) async fn index(pool: web::Data<DbPool>) -> &'static str {
    "<p>Hello</p>"
}
