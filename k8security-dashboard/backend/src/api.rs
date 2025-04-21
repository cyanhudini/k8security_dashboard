
use actix_web::{web, HttpResponse, Responder};

use backend::{fetch_all_vuln_entries, fetch_receiver_emails, bulk_add_vulns, create_email_entry, filter_vuln_entries_by_severity};
use crate::models::{NewEmail, FilterQuery};
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

pub(crate) async fn post_filter_query(pool : web::Data<DbPool>, req: web::Json<FilterQuery>) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    let severity_filters = req.into_inner().query;

    let response = web::block(move ||{
        let mut connection = pool.get().unwrap();

        filter_vuln_entries_by_severity(&mut connection, severity_filters)
    })
    .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

pub(crate) async fn post_new_vulns(pool : web::Data<DbPool>) ->actix_web::Result<impl Responder>{
    let pool = pool.clone();
    web::block(move ||{
        let mut connection = pool.get().unwrap();

        // TODO: Result muss besser genutzt werden
        let _ = bulk_add_vulns(&mut connection);
    })
    .await?;

    Ok(HttpResponse::Ok().json("Successfully inserted"))
}

pub(crate) async fn post_new_email_adress(pool : web::Data<DbPool>, req: web::Json<NewEmail>) -> actix_web::Result<impl Responder>{
    let pool = pool.clone();
    let new_email_adress = req.into_inner().email_adress;

    web::block(move ||{
        let mut connection = pool.get().unwrap();
        //TODO: Result muss genutzt werden
        let _ = create_email_entry(&mut connection, new_email_adress);
    })
    .await?;

    Ok(HttpResponse::Ok().json("Successfully inserted"))
}

pub(crate) async fn post_vuln_flter(pool : web::Data<DbPool>, req: web::Json<NewEmail>){
    
}

pub(crate) async fn delete_vulns(pool : web::Data<DbPool>){}

pub(crate) async fn delete_receiver_email(pool : web::Data<DbPool>){}

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
    // TODO: add HTTPResponse:Body
}
