set dotenv-load := true
alias m := migrate-db
alias g := gen-db-entity

migrate-db drop="":
  @echo 'running migration {{drop}}'
  cargo run --manifest-path=migration/Cargo.toml -- {{drop}}

gen-db-entity:
  sea-orm-cli generate entity -u $DB_CONNECTION/$DB_NAME -o src/database

create-env:
  echo 'export API_PORT=3000\nexport API_URI=http://localhost\nexport JWT_SECRET=your-secret\nexport DB_NAME=todo_server_axum\nexport DB_CONNECTION=postgres://postgres:password@localhost:5432' > .env