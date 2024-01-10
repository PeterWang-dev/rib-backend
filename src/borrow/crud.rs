use super::*;
use crate::{
    book,
    error::{Error::OperationError, OperationError as OpErr, Result},
};
use sea_orm::{ConnectionTrait, DbConn, Statement};

pub async fn create(db: &DbConn, builder: BorrowBuilder) -> Result<()> {
    let book = book::read_book(db, &builder.book_uuid).await?;

    let mut book_builder = book::BookBuilder::from_model(book);
    book_builder.set_borrowed_by(builder.user_id);
    book::update_book(db, &builder.book_uuid, book_builder).await?;

    Ok(())
}

pub async fn read_by_user_id(db: &DbConn, user_id: i32) -> Result<Vec<Model>> {
    let query_stmt = Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
            SELECT
                "user_id",
                "book_id",
                "book_title",
                "borrow_date",
                "return_date",
                "is_renewed"
            FROM
                "borrow_view"
            GROUP BY
                "user_id",
            WHERE
                "user_id" = "{}";"#,
        [user_id.into()],
    );

    match Model::find_by_statement(query_stmt).all(db).await {
        Ok(borrows) => Ok(borrows),
        Err(_) => Err(OperationError(OpErr::ObjectNotFound)),
    }
}

pub async fn update_renew(db: &DbConn, book_uuid: Uuid) -> Result<()> {
    let book = book::read_book(db, &book_uuid).await?;

    if book.borrowed_by.is_none() {
        return Err(OperationError(OpErr::ObjectNotFound));
    }

    let mut book_builder = book::BookBuilder::from_model(book);
    book_builder.set_renewed();
    book::update_book(db, &book_uuid, book_builder).await?;

    Ok(())
}

pub async fn delete(db: &DbConn, book_uuid: Uuid) -> Result<()> {
    let book = book::read_book(db, &book_uuid).await?;

    if book.borrowed_by.is_none() {
        return Err(OperationError(OpErr::ObjectNotFound));
    }

    let mut book_builder = book::BookBuilder::from_model(book);
    book_builder.set_returned();
    book::update_book(db, &book_uuid, book_builder).await?;

    Ok(())
}
