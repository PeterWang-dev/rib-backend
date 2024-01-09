use sea_orm_migration::prelude::*;
use std::env;

const DATABASE_URL: &str = "sqlite://rib-db.sqlite3?mode=rwc";

#[async_std::main]
async fn main() {
    env::set_var("DATABASE_URL", DATABASE_URL);
    cli::run_cli(migration::Migrator).await;
}
