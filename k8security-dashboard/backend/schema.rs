// @generated automatically by Diesel CLI.

diesel::table! {
    emails (id) {
        id -> Int4,
        email_adress -> Varchar,
        receiving -> Nullable<Bool>,
    }
}

diesel::table! {
    vulnerabilities (id) {
        id -> Int4,
        vulnerability_id -> Nullable<Text>,
        severity -> Nullable<Text>,
    }
}

diesel::table! {
    vulnerability (id) {
        id -> Int4,
        vuln_id -> Varchar,
        pkg_name -> Varchar,
        installed_version -> Varchar,
        severity -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    emails,
    vulnerabilities,
    vulnerability,
);
