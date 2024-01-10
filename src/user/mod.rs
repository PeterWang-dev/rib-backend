mod crud;

pub use crud::{
    create as create_user, delete as delete_user, read_by_id as read_user,
    read_by_identifier as read_user_by_identifier, update as update_user,
};

use entity::user::{ActiveModel, Model};
use sea_orm::{prelude::DateTimeLocal, ActiveValue, IntoActiveModel};

#[derive(Clone, Debug)]
pub enum UserIdentifier {
    PhoneNumber(String),
    EmailAddress(String),
}

#[derive(Debug)]
pub struct UserBuilder {
    active_model: ActiveModel,
}

impl UserBuilder {
    pub fn new(identifier: UserIdentifier, password: String, role_id: i32) -> Self {
        match identifier {
            UserIdentifier::PhoneNumber(phone_number) => Self {
                active_model: ActiveModel {
                    phone_number: ActiveValue::Set(Some(phone_number)),
                    password: ActiveValue::Set(password),
                    role_id: ActiveValue::Set(role_id),
                    ..Default::default()
                },
            },
            UserIdentifier::EmailAddress(email_address) => Self {
                active_model: ActiveModel {
                    email_address: ActiveValue::Set(Some(email_address)),
                    password: ActiveValue::Set(password),
                    role_id: ActiveValue::Set(role_id),
                    ..Default::default()
                },
            },
        }
    }

    pub fn from_model(model: Model) -> Self {
        Self {
            active_model: model.into_active_model(),
        }
    }

    pub fn set_password(&mut self, password: String) {
        self.active_model.password = ActiveValue::Set(password);
    }

    pub fn set_name(&mut self, first_name: String, last_name: String) {
        self.active_model.first_name = ActiveValue::Set(Some(first_name));
        self.active_model.last_name = ActiveValue::Set(Some(last_name));
    }

    pub fn set_birthday(&mut self, birthday: DateTimeLocal) {
        self.active_model.birthday = ActiveValue::Set(Some(birthday.to_string()));
    }

    pub fn set_address(&mut self, address: String) {
        self.active_model.address = ActiveValue::Set(Some(address));
    }

    pub fn set_phone_number(&mut self, phone_number: String) {
        self.active_model.phone_number = ActiveValue::Set(Some(phone_number));
    }

    pub fn set_email_address(&mut self, email_address: String) {
        self.active_model.email_address = ActiveValue::Set(Some(email_address));
    }
}
