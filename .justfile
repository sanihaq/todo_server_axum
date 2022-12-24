set dotenv-load := true
alias g := gen-db-entity

gen-db-entity:
  sea-orm-cli generate entity -u $DB_CONNECTION/$DB_NAME -o src/database