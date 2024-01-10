use crate::{
    error::{
        Error::{LoginError, OperationError},
        LoginError as LoginErr, OperationError as OpErr, Result,
    },
    user::{self, UserIdentifier},
};
use sea_orm::DbConn;

#[derive(Debug, Clone, Copy)]
pub enum Role {
    Administrator = 1,
    Librarian = 2,
    Reader = 3,
}

pub struct State {
    db_conn: DbConn,
    login_user: Option<i32>,
    role: Option<Role>,
}

impl State {
    pub fn new(db_conn: DbConn) -> Self {
        Self {
            db_conn,
            login_user: None,
            role: None,
        }
    }

    pub fn db_conn(&self) -> &DbConn {
        &self.db_conn
    }

    pub fn login_user(&self) -> Result<i32> {
        match self.login_user {
            Some(user_id) => Ok(user_id),
            None => Err(OperationError(OpErr::InvalidOperation)),
        }
    }

    pub fn role(&self) -> Result<Role> {
        match self.role {
            Some(role) => Ok(role),
            None => Err(OperationError(OpErr::InvalidOperation)),
        }
    }

    pub async fn set_login(&mut self, identifier: &UserIdentifier, password: String) -> Result<()> {
        let conn = &self.db_conn;
        let (user_id, role) = match user::read_user_by_identifier(conn, &identifier).await {
            Ok(user) => {
                if user.password == password {
                    match user.role_id {
                        1 => (user.id, Role::Administrator),
                        2 => (user.id, Role::Librarian),
                        3 => (user.id, Role::Reader),
                        _ => return Err(OperationError(OpErr::InvalidOperation)),
                    }
                } else {
                    return Err(LoginError(LoginErr::InvalidPassword));
                }
            }
            Err(OperationError(OpErr::ObjectNotFound)) => {
                return Err(LoginError(LoginErr::InvalidUsername))
            }
            Err(e) => return Err(e),
        };

        self.login_user = Some(user_id);
        self.role = Some(role);
        Ok(())
    }
}
