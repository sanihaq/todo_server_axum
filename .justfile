set dotenv-load := true
alias m := migrate-db
alias g := gen-db-entity

migrate-db:
  DATABASE_URL=$DB_CONNECTION/$DB_NAME sea-orm-cli migrate refresh

gen-db-entity:
  sea-orm-cli generate entity -u $DB_CONNECTION/$DB_NAME -o src/database