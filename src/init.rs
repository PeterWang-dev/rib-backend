use crate::{
    error::{Error as RibError, InitializeError::*, Result as RibResult},
    state::State,
    user::{self, UserBuilder, UserIdentifier},
};
use log::{error, info, warn};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbConn, DbErr};
use std::{env, fs::File, io::ErrorKind, str::FromStr};

const DATABASE_PATH: &str = "rib-db.sqlite3";

pub async fn initialize() -> RibResult<State> {
    info!("Welcome to Ribrarian Backend! Initializing...");

    // Check if database exists
    if let Err(err) = File::open(DATABASE_PATH) {
        if err.kind() == ErrorKind::NotFound {
            warn!("Database not found, assuming first run...");
            return Err(RibError::FirstRun);
        }
    }

    let connect_url = format!("sqlite://{}?mode=rw", DATABASE_PATH);

    match connect_database_with_logging(&connect_url).await {
        Ok(db) => {
            info!("Initialization completed successfully.");
            Ok(State::new(db))
        }
        Err(e) => {
            error!("Initialization failed: {}", e);
            Err(RibError::InitializeError(DatabaseConnectError(e)))
        }
    }
}

pub async fn setup(admin_identifier: UserIdentifier, admin_password: String) -> RibResult<State> {
    info!("Ribrarian Backend Hello! Assumed first run, setting up...");

    let db_conn = match create_database().await {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to create database: {}", e);
            return Err(RibError::InitializeError(DatabaseCreateError(e)));
        }
    };

    if let Err(err) = crate_schema(&db_conn).await {
        error!("Failed to create schema: {}", err);
        return Err(RibError::InitializeError(SchemaCreateError(err)));
    }

    if let Err(err) = create_admin(&db_conn, &admin_identifier, &admin_password).await {
        error!("Failed to create Administrator: {:#?}", err);
        return Err(err);
    }

    info!("Setup completed successfully.");
    Ok(State::new(db_conn))
}

// Connect to database
async fn connect_database_with_logging(db_url: &str) -> Result<DbConn, DbErr> {
    info!("Connecting database...");

    let mut opt = ConnectOptions::new(db_url);

    // Set up sqlx log_level
    let log_level = env::var("RIB_LOG").unwrap_or("info".to_owned());
    opt.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::from_str(&log_level).unwrap());

    match Database::connect(opt).await {
        Ok(db) => {
            info!("Database connected successfully.");
            Ok(db)
        }
        Err(e) => {
            warn!("Failed to connect database: {}", e);
            Err(e)
        }
    }
}

// Initialize database
async fn create_database() -> Result<DbConn, DbErr> {
    info!("Creating database...");

    let create_url = format!("sqlite://{}?mode=rwc", DATABASE_PATH);

    match connect_database_with_logging(&create_url).await {
        Ok(db) => {
            info!("Database created successfully.");
            Ok(db)
        }
        Err(e) => {
            warn!("Failed to create database: {}", e);
            Err(e)
        }
    }
}

async fn crate_schema(db_conn: &DbConn) -> Result<(), DbErr> {
    info!("Creating and initializing tables...");

    match Migrator::up(db_conn, Some(1)).await {
        Ok(_) => {
            info!("Tables created and initialized successfully.");
            Ok(())
        }
        Err(e) => {
            warn!("Failed to create and initialize tables: {}", e);
            return Err(e);
        }
    }
}

async fn create_admin(
    db_conn: &DbConn,
    identifier: &UserIdentifier,
    password: &str,
) -> Result<(), RibError> {
    info!("Creating Administrator...");

    let builder = UserBuilder::new(identifier.clone(), password.to_owned(), 1);

    match user::create_user(db_conn, builder).await {
        Ok(_) => {
            info!("Administrator created successfully.");
            Ok(())
        }
        Err(e) => {
            warn!("Failed to create Administrator: {:#?}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    async fn test_connect_database() {}

    #[tokio::test]
    async fn test_create_database() {}

    #[tokio::test]
    async fn test_crate_schema() {}

    #[tokio::test]
    async fn test_create_admin() {}
}

#[cfg(test)]
mod intergrated_test {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test() {
        let state = match initialize().await {
            Ok(state) => state,
            Err(RibError::FirstRun) => {
                match setup(
                    UserIdentifier::PhoneNumber("18471776321".to_owned()),
                    "test@123".to_owned(),
                )
                .await
                {
                    Ok(state) => state,
                    Err(e) => {
                        panic!("Failed to setup: {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Failed to initialize: {:?}", e);
            }
        };
    }
}
