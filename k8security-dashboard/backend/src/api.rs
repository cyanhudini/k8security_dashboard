use actix_web::{web, HttpResponse, Responder};

use crate::models::{DeleteVulnRequest, FilterQuery, NewEmail, SetEmailQuery};
use crate::DbPool;
use backend::*;
use serde::Serialize;

pub(crate) async fn get_all_vulns(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();

    let all_vulns = web::block(move || {
        let mut connection = pool.get().unwrap();

        fetch_all_vuln_entries(&mut connection)
    })
    .await?;

    Ok(web::Json(all_vulns))
}

pub(crate) async fn post_filter_query(
    pool: web::Data<DbPool>,
    req: web::Json<FilterQuery>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    let severity_filters = req.into_inner().query.unwrap_or(vec!["ALL".to_string()]);

    let response = web::block(move || {
        let mut connection = pool.get().unwrap();
        filter_vuln_entries_by_severity(&mut connection, severity_filters)
    })
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

pub(crate) async fn post_new_vulns(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    web::block(move || {
        let mut connection = pool.get().unwrap();

        // TODO: Result muss besser genutzt werden
        let _ = add_vulns_from_file(&mut connection);
    })
    .await?;

    Ok(HttpResponse::Ok().json("Successfully inserted"))
}

pub(crate) async fn post_new_email_adress(
    pool: web::Data<DbPool>,
    req: web::Json<NewEmail>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    let new_email_adress = req.into_inner().email_adress;

    web::block(move || {
        let mut connection = pool.get().unwrap();
        //TODO: Result muss genutzt werden
        create_email_entry(&mut connection, new_email_adress)
    })
    .await?;

    Ok(HttpResponse::Ok().json("Successfully inserted"))
}

pub(crate) async fn delete_vulns(
    pool: web::Data<DbPool>,
    req: web::Json<Vec<DeleteVulnRequest>>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();

    let vuln_obj = req.into_inner();
    print!("Delete vuln request: {:?}", vuln_obj);

    let response = web::block(move || {
        let mut connection = pool.get().unwrap();
        let vuln_ids: Vec<i32> = vuln_obj.into_iter().map(|v| v.id).collect();
        delete_vuln_entry(&mut connection, vuln_ids)
    })
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

pub(crate) async fn delete_receiver_email(pool: web::Data<DbPool>) {}

pub(crate) async fn get_all_receiver_emails(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    let response = web::block(move || {
        let mut connection = pool.get().unwrap();

        fetch_receiver_emails(&mut connection)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}

pub(crate) async fn update_status_email(
    pool: web::Data<DbPool>,
    req: web::Json<SetEmailQuery>,
) -> actix_web::Result<impl Responder> {
    print!("{:?}", req);
    let email_id = req.into_inner().email_id;
    let pool = pool.clone();
    let response = web::block(move || {
        let mut connection: r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        > = pool.get().unwrap();

        update_email_entry(&mut connection, email_id)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}

// group by scan type
pub(crate) async fn fetch_all_vulns_then_group(
    pool: web::Data<DbPool>,
    req: web::Json<FilterQuery>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    let filter = req.into_inner().query.unwrap_or(vec!["ALL".to_string()]);
    print!("Fetching vulnerabilities grouped by scan type...");

    let result = web::block(move || {
        let mut connection = pool.get().unwrap();

        group_by_docker_scan_type(&mut connection, filter)
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}
pub(crate) async fn fetch_all_vulns_then_group_by_pkg(
    pool: web::Data<DbPool>,
    req: web::Json<FilterQuery>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
    let filter = req.into_inner().query.unwrap_or(vec!["ALL".to_string()]);

    let result = web::block(move || {
        let mut connection = pool.get().unwrap();

        group_by_pkgid_pkgname(&mut connection)
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}

// generic function für alle Create, Update, Delete Operationen
pub(crate) async fn run_rud_db_task<F, T>(
    pool: web::Data<DbPool>,
    f: F,
) -> actix_web::Result<impl Responder>
where
    F: FnOnce(
            &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
        ) -> T
        + Send
        + 'static,
    T: Send + Serialize + 'static,
{
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
pub(crate) async fn run_create_db_task<F, T>(
    pool: web::Data<DbPool>,
    f: F,
) -> actix_web::Result<impl Responder>
where
    F: FnOnce(
            &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
        ) -> T
        + Send
        + 'static,
    T: Send + Serialize + 'static,
{
    let pool = pool.clone();
    let response = web::block(move || {
        let mut connection = pool.get().unwrap();
        f(&mut connection)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}
