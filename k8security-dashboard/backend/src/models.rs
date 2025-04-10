use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Vulnerability{

}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "vulnerabilities"]
pub struct NewVulnerability{

}


