docker exec -it postgres_k8s psql -U admin -d vulndb


## Dependencies/ Tech Stack

- **Backend**: Rust, Actix-Web, Diesel, PostgreSQL
- **Frontend** : React, TailwindCSS, VITE
- **Database** : PostgreSQL (Container)
- **Software** : Trivy Scanner : ```curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sudo sh -s -- -b /usr/local/bin v0.61.1 ```

## Setup

#### Backend Setup

###### Database

- cd docker-postgresql
- docker-compose up &

###### Trivy Report

- 