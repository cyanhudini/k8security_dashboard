diesel::table! {
    vulnerability (id) {
        id -> Int4,
        vuln_id -> Varchar,
        pkg_name -> Varchar,
        installed_version -> Varchar,
        severity -> Varchar,
    }
    
}

diesel::table! {
    emails (id) {
        id -> Int4,
        email_adress -> Varchar,
        receiving -> Bool,
    }
}

