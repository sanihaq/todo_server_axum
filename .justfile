set dotenv-load := true
alias m := migrate-db
alias g := gen-db-entity

migrate-db drop="":
  @echo 'running migration {{drop}}'
  cargo run --manifest-path=migration/Cargo.toml -- {{drop}}

gen-db-entity:
  sea-orm-cli generate entity -u $DB_CONNECTION/$DB_NAME -o src/database