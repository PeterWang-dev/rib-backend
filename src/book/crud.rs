use super::*;
use crate::error::{
    Error::{DatabaseError, OperationError},
    OperationError as OpErr, Result,
};
use entity::book::{Model, Entity};
use sea_orm::{DbConn, EntityTrait};

pub async fn insert(db: &DbConn, book_builder: BookBuilder) -> Result<Uuid> {
    match Entity::insert(book_builder.model).exec(db).await {
        Ok(book) => Ok(Uuid::parse_str(book.last_insert_id.as_str()).unwrap()),
        Err(e) => Err(DatabaseError(e)),
    }
}

pub async fn read_by_identifier(db: &DbConn, uuid: Uuid) -> Result<Model> {
    match Entity::find_by_id(uuid.to_string()).one(db).await {
        Ok(Some(book)) => Ok(book),
        Ok(None) => Err(OperationError(OpErr::ObjectNotFound)),
        Err(e) => Err(DatabaseError(e)),
    }
}

pub async fn update(db: &DbConn, uuid: Uuid, book_builder: BookBuilder) -> Result<()> {
    let book = read_by_identifier(db, uuid).await?;
    let mut new_model = book_builder.model;
    new_model.uuid = ActiveValue::Set(book.uuid);

    match Entity::update(new_model).exec(db).await {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError(e)),
    }
}

pub async fn delete(db: &DbConn, uuid: Uuid) -> Result<u64> {
    match Entity::delete_by_id(uuid.to_string()).exec(db).await {
        Ok(res) => Ok(res.rows_affected),
        Err(e) => Err(DatabaseError(e)),
    }
}
