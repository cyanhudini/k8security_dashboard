
## Dependencies/ Tech Stack

- **Backend**: Rust, Actix-Web, Diesel, PostgreSQL
- **Frontend** : React, TailwindCSS, VITE
- **Database** : PostgreSQL (Container)
- **Software** : Trivy Scanner : ```curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sudo sh -s -- -b /usr/local/bin v0.61.1 ```

## Setup

#### Backend

- cargo run (dies sollte alle Pakete aus cargo.toml installieren)
- erstelle eine .env mit DATABASE_URL=postgres://nutzer:passwort@localhost:5432/vulndb (nutzer/passwort im docker compose festlegen)

- im backend Ordner: diesel setup
- dann diesel migration run 

#### Frontend

- npm install
- npm run dev

#### Database

- cd docker-postgresql
- docker-compose up &

#### Trivy Report

Generiere einen Trivy Report mittels trivy k8s --format json -o results.json und rufe dann den Endpunkt localhost:8080/add_vulns_bulk.
Dies ist der vorläufige Weg, später wird dies durch ein Skript ersetzt welches einmal pro Tag den Trivy Scanner laufen lässt um diesen dann später in die Datenbank einzuführen.
