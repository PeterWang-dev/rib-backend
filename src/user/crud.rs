use super::*;
use crate::error::{
    Error::{DatabaseError, OperationError},
    OperationError as OpErr, Result,
};
use entity::user::{self, Entity, Model};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};

pub async fn create(db: &DbConn, user_builder: UserBuilder) -> Result<i32> {
    match Entity::insert(user_builder.active_model).exec(db).await {
        Ok(user) => Ok(user.last_insert_id),
        Err(e) => Err(DatabaseError(e)),
    }
}

pub async fn read_by_identifier(db: &DbConn, identifier: &UserIdentifier) -> Result<Model> {
    let res = match identifier {
        UserIdentifier::PhoneNumber(phone_number) => read_by_phone(db, &phone_number).await,
        UserIdentifier::EmailAddress(email_address) => read_by_email(db, &email_address).await,
    };

    match res {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub async fn read_by_id(db: &DbConn, id: i32) -> Result<Model> {
    match Entity::find_by_id(id).one(db).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(OperationError(OpErr::ObjectNotFound)),
        Err(e) => Err(DatabaseError(e)),
    }
}

pub async fn update(db: &DbConn, id: i32, user_builder: UserBuilder) -> Result<()> {
    let user = read_by_id(db, id).await?;
    let mut new_model = user_builder.active_model;
    new_model.id = ActiveValue::Set(user.id);

    match Entity::update(new_model).exec(db).await {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError(e)),
    }
}

pub async fn delete(db: &DbConn, id: i32) -> Result<u64> {
    match Entity::delete_by_id(id).exec(db).await {
        Ok(res) => Ok(res.rows_affected),
        Err(e) => Err(DatabaseError(e)),
    }
}

async fn read_by_phone(db: &DbConn, phone_number: &str) -> Result<Model> {
    match Entity::find()
        .filter(user::Column::PhoneNumber.eq(phone_number))
        .one(db)
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(OperationError(OpErr::ObjectNotFound)),
        Err(e) => Err(DatabaseError(e)),
    }
}

async fn read_by_email(db: &DbConn, email_address: &str) -> Result<Model> {
    match Entity::find()
        .filter(user::Column::EmailAddress.eq(email_address))
        .one(db)
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(OperationError(OpErr::ObjectNotFound)),
        Err(e) => Err(DatabaseError(e)),
    }
}
