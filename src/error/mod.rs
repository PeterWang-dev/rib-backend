mod init;
mod login;
mod oper;

pub use init::InitializeError;
pub use login::LoginError;
pub use oper::OperationError;

use sea_orm::DbErr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FirstRun,
    InitializeError(InitializeError),
    LoginError(LoginError),
    OperationError(OperationError), //? Is this necessary?
    DatabaseError(DbErr),
}
