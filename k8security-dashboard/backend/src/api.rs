
use actix_web::{web, HttpResponse, Responder};

use backend::{bulk_add_vulns, create_email_entry, fetch_all_vuln_entries, fetch_receiver_emails, filter_vuln_entries_by_severity, update_email_entry, group_by_pkgid_pkgname, get_grouped_by_docker_scan_type};
use serde::Serialize;
use crate::models::{NewEmail, FilterQuery, SetEmailQuery};
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
    let severity_filters = req.into_inner().query.unwrap_or(vec!["ALL".to_string()]);

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
        create_email_entry(&mut connection, new_email_adress)
    })
    .await?;

    Ok(HttpResponse::Ok().json("Successfully inserted"))
}

pub(crate) async fn delete_vulns(pool : web::Data<DbPool>){}

pub(crate) async fn delete_receiver_email(pool : web::Data<DbPool>){}

pub(crate) async fn get_all_receiver_emails(pool : web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    let response = web::block(move ||{
        let mut connection = pool.get().unwrap();

        fetch_receiver_emails(&mut connection)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}

pub(crate) async fn update_status_email(pool : web::Data<DbPool>, req: web::Json<SetEmailQuery>) -> actix_web::Result<impl Responder> {
    print!("{:?}", req);
    let email_id = req.into_inner().email_id;
    let pool = pool.clone();
    let response = web::block(move ||{
        let mut connection: r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>> = pool.get().unwrap();
        
        update_email_entry(&mut connection, email_id)

    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))

}

pub(crate) async fn fetch_all_vulns_then_group(pool : web::Data<DbPool>, req: web::Json<FilterQuery>) -> actix_web::Result<impl Responder>{
    let pool = pool.clone();
    let filter = req.into_inner().query.unwrap_or(vec!["ALL".to_string()]);
    let result = web::block(move ||{
        let mut connection = pool.get().unwrap();

        group_by_pkgid_pkgname(&mut connection)
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}


// generic function für alle Create, Update, Delete Operationen
pub(crate) async fn run_rud_db_task<F, T>(pool : web::Data<DbPool>, f: F) -> actix_web::Result<impl Responder>
    where F: FnOnce(&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>> ) -> T + Send + 'static,
          T: Send + Serialize + 'static {
            let pool = pool.clone();
            let response = web::block(move || {  
                let mut connection = pool.get().unwrap();
                f(&mut connection)
            })
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(response))

}

// generic function für alle Read Operationen
pub(crate) async fn run_create_db_task<F, T>(pool : web::Data<DbPool>, f: F) -> actix_web::Result<impl Responder>
    where F: FnOnce(&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>> ) -> T + Send + 'static,
          T: Send + Serialize + 'static {
            let pool = pool.clone();
            let response = web::block(move || {  
                let mut connection = pool.get().unwrap();
                f(&mut connection)
            })
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(response))

}
