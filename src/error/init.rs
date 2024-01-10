use sea_orm::DbErr;

#[derive(Debug)]
pub enum InitializeError {
    DatabaseConnectError(DbErr),
    DatabaseCreateError(DbErr),
    SchemaCreateError(DbErr),
    PrivilegeInitializeError(DbErr),
}