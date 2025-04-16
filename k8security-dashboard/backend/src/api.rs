
use actix_web::{web, HttpResponse, Responder};
use backend::{fetch_all_vuln_entries, fetch_receiver_emails, bulk_add_vulns};
use crate::DbPool;


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

        //
        let _ = bulk_add_vulns(&mut connection);
    })
    .await?;

    Ok(HttpResponse::Ok().json("Successfully inserted"))

}

pub(crate) async fn get_all_receiver_emails(pool : web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    let response = web::block(move ||{
        let mut connection = pool.get().unwrap();

        fetch_receiver_emails(&mut connection)
    })
    .await?;

    Ok(HttpResponse::Ok().json(response))
}



pub(crate) async fn index(pool: web::Data<DbPool>) -> &'static str {
    "<p>Hello</p>"
}
