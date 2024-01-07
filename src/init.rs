use crate::{
    state::State,
    entity::{prelude::*, privilege},
    error::{Error as RibError, InitializeError::*, Result as RibResult},
};
use log::{error, info, trace, warn};
use sea_orm::{
    ActiveValue, ConnectOptions, ConnectionTrait, Database, DbConn, DbErr, EntityTrait, Schema,
};
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

pub async fn setup(
    admin_password: String,
    first_name: String,
    last_name: String,
    phone_number: String,
) -> RibResult<State> {
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

    if let Err(err) = create_admin(admin_password, first_name, last_name, phone_number).await {
        error!("Failed to create Administrator: {}", err);
        return Err(RibError::InitializeError(PrivilegeInitializeError(err)));
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
    info!("Creating tables...");

    let builder = db_conn.get_database_backend();
    let schema = Schema::new(builder);

    trace!("Builder: {:#?}\nSchema: {:#?}", builder, schema);

    let statements = vec![
        builder.build(&schema.create_table_from_entity(Privilege)),
        builder.build(&schema.create_table_from_entity(User)),
        builder.build(&schema.create_table_from_entity(Book)),
    ];

    for statement in statements {
        trace!("Executing statement: {}", statement);

        if let Err(err) = db_conn.execute(statement).await {
            warn!("Failed to execute statement: {}", err);
            return Err(err);
        }
    }

    info!("Tables created successfully.");

    info!("Initializing privileges...");

    let privileges = vec![
        privilege::ActiveModel {
            id: ActiveValue::set(0),
            description: ActiveValue::set("Administrator".to_owned()),
        },
        privilege::ActiveModel {
            id: ActiveValue::set(1),
            description: ActiveValue::set("Librarian".to_owned()),
        },
        privilege::ActiveModel {
            id: ActiveValue::set(2),
            description: ActiveValue::set("Reader".to_owned()),
        },
    ];

    for privilege in privileges {
        trace!("Inserting privilege: {:#?}", privilege);

        if let Err(err) = Privilege::insert(privilege).exec(db_conn).await {
            warn!("Failed to insert privilege: {}", err);
            return Err(err);
        }
    }

    info!("Privileges initialized successfully.");

    Ok(())
}

async fn create_admin(
    password: String,
    first_name: String,
    last_name: String,
    phone_number: String,
) -> Result<(), DbErr> {
    info!("Creating Administrator...");

    todo!("create_admin");
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
    async fn first_run() {}

    #[tokio::test]
    #[serial]
    async fn next_run() {}
}
