pub mod crud;

use std::str::FromStr;

use chrono::{Duration, Local};
use entity::book::ActiveModel;
use sea_orm::{prelude::DateTimeLocal, ActiveValue, TryIntoModel};
use uuid::Uuid;

pub struct BookBuilder {
    model: ActiveModel,
}

impl BookBuilder {
    pub fn new(
        title: String,
        author: String,
        publisher: String,
        published_time: DateTimeLocal,
        category: String,
        isbn: String,
    ) -> Self {
        Self {
            model: ActiveModel {
                uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
                title: ActiveValue::Set(title),
                author: ActiveValue::Set(author),
                publisher: ActiveValue::Set(publisher),
                published_time: ActiveValue::Set(published_time.to_string()),
                category: ActiveValue::Set(category),
                isbn: ActiveValue::Set(isbn),
                ..Default::default()
            },
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.model.title = ActiveValue::Set(title);
    }

    pub fn set_author(&mut self, author: String) {
        self.model.author = ActiveValue::Set(author);
    }

    pub fn set_publisher(&mut self, publisher: String) {
        self.model.publisher = ActiveValue::Set(publisher);
    }

    pub fn set_published_time(&mut self, published_time: DateTimeLocal) {
        self.model.published_time = ActiveValue::Set(published_time.to_string());
    }

    pub fn set_category(&mut self, category: String) {
        self.model.category = ActiveValue::Set(category);
    }

    pub fn set_isbn(&mut self, isbn: String) {
        self.model.isbn = ActiveValue::Set(isbn);
    }

    pub fn set_borrowed_by(&mut self, borrowed_by: i32) {
        let model = &mut self.model;
        let now = Local::now();
        let ret = now + Duration::days(30);

        model.borrowed_by = ActiveValue::Set(Some(borrowed_by));
        model.borrowed_date = ActiveValue::Set(Some(now.to_string()));
        model.return_date = ActiveValue::Set(Some(ret.to_string()));
    }

    pub fn set_renewed(&mut self) {
        let model = &mut self.model;
        let old_ret = model
            .clone()
            .try_into_model()
            .unwrap()
            .return_date
            .unwrap();
        let old_ret = DateTimeLocal::from_str(&old_ret).unwrap();
        let ret = old_ret + Duration::days(30);

        model.return_date = ActiveValue::Set(Some(ret.to_string()));
        model.is_renewed = ActiveValue::Set(true);
    }

    pub fn set_returned(&mut self) {
        let model = &mut self.model;

        model.borrowed_by = ActiveValue::Set(None);
        model.borrowed_date = ActiveValue::Set(None);
        model.return_date = ActiveValue::Set(None);
        model.is_renewed = ActiveValue::Set(false);
    }
}
