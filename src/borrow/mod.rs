mod crud;

pub use crud::{
    create as borrow_book, delete as return_book, read_by_user_id as user_borrows,
    update_renew as renew_book,
};

use sea_orm::{prelude::DateTimeLocal, FromQueryResult};
use uuid::Uuid;

#[derive(Debug, FromQueryResult)]
pub struct Model {
    pub user_id: i32,
    pub book_uuid: Uuid,
    pub book_title: String,
    pub borrow_date: DateTimeLocal,
    pub return_date: DateTimeLocal,
    pub is_renewed: bool,
}

pub struct BorrowBuilder {
    user_id: i32,
    book_uuid: Uuid,
}

impl BorrowBuilder {
    pub fn new(user_id: i32, book_uuid: Uuid) -> Self {
        Self { user_id, book_uuid }
    }
}
