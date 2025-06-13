// tests for the backend module
use crate::api::{add_vulns_from_file, create_email_entry, update_email_entry};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Deserialize;
use std::sync::Arc;
use std::sync::Mutex;

use crate::models::{Emails, NewEmail, SetEmailQuery, Vulnerability};
use crate::schema::{emails, vulnerability};
use crate::tests::helpers::{setup_test_db, teardown_test_db};
use crate::DbPool;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        delete_receiver_email, delete_vulns, get_all_receiver_emails, post_new_email_adress,
        post_new_vulns,
    };
    use crate::models::{NewVulnerability, SetEmailQuery};

    #[actix_web::test]
    async fn test_post_new_vulns() {
        let pool = setup_test_db().await;
        let new_vuln = NewVulnerability {
            vuln_id: "test_vuln".to_string(),
            origin: "test_origin".to_string(),
            severity: "HIGH".to_string(),
            description: "Test vulnerability".to_string(),
        };

        let response = post_new_vulns(web::Data::new(pool), web::Json(new_vuln)).await;
        assert!(response.is_ok());
    }

    #[actix_web::test]
    async fn test_post_new_email_adress() {
        let pool = setup_test_db().await;
        let new_email = NewEmail {
            email_adress: "test@example.com".to_string(),
        };

        let response = post_new_email_adress(web::Data::new(pool), web::Json(new_email)).await;
        assert!(response.is_ok());
        let response = delete_receiver_email(web::Data::new(pool), web::Json(new_email)).await;
        assert!(response.is_ok());
        let emails: Vec<Emails> = response.unwrap().into_inner();
        assert!(!emails.is_empty());
    }
    #[actix_web::test]
    async fn test_delete_receiver_email() {
        let pool = setup_test_db().await;
        let new_email = NewEmail {
            email_adress: "test@example.com".to_string(),
        };
        let response = delete_receiver_email(web::Data::new(pool), web::Json(new_email)).await;
        assert!(response.is_ok());
    }
}
