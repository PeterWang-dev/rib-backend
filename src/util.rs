// // Administrator Operations
// // User Ops
// pub async fn add_user(state: &State, user_info: UserInfo) -> Result<()> {
//     //? State check can be destructured
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Administrator => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     match user_info.insert_db(state.db_conn()).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// pub async fn find_user(state: &State, phone_number: String) -> Result<UserInfo> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Administrator => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     match UserInfo::from_phone_number_db(&phone_number, state.db_conn()).await {
//         Ok(Some(user_info)) => Ok(user_info),
//         Ok(None) => Err(Error::UserNotFound),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// pub async fn delete_user(state: &State, user_info: UserInfo) -> Result<()> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Administrator => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     match user_info.delete_db(state.db_conn()).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// pub async fn update_user(state: &State, new_info: UserInfo) -> Result<UserInfo> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Administrator => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     match new_info.update_db(state.db_conn()).await {
//         Ok(info) => Ok(info),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// // BookOps
// pub async fn add_book(state: &State, book_info: BookInfo) -> Result<BookInfo> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Administrator => (),
//             Role::Librarian => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     match book_info.insert_db(state.db_conn()).await {
//         Ok(info) => Ok(info),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// pub async fn delete_book(state: &State, book_info: BookInfo) -> Result<()> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Administrator => (),
//             Role::Librarian => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     match book_info.delete_db(state.db_conn()).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// pub async fn find_book(state: &State, uuid: Uuid) -> Result<BookInfo> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Administrator => (),
//             Role::Librarian => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     match BookInfo::from_uuid_db(uuid, state.db_conn()).await {
//         Ok(Some(book_info)) => Ok(book_info),
//         Ok(None) => Err(Error::BookNotFound),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// pub async fn update_book(state: &State, new_info: UserInfo) -> Result<UserInfo> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Administrator => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     match new_info.update_db(state.db_conn()).await {
//         Ok(info) => Ok(info),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// // Librarian Operations
// // Book Ops
// // add_book, delete_book, find_book
// pub async fn borrow_book(state: &State, user_phone_number: String, book_uuid: Uuid) -> Result<()> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Librarian => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     let user_id = match UserInfo::from_phone_number_db(&user_phone_number, state.db_conn()).await {
//         Ok(Some(user)) => user.id(),
//         Ok(None) => return Err(Error::UserNotFound),
//         Err(e) => return Err(Error::DatabaseError(e)),
//     };

//     let mut book = match BookInfo::from_uuid_db(book_uuid, state.db_conn()).await {
//         Ok(Some(book)) => book,
//         Ok(None) => return Err(Error::BookNotFound),
//         Err(e) => return Err(Error::DatabaseError(e)),
//     };

//     let borrow = Borrow::new(
//         user_id,
//         chrono::Local::now(),
//         chrono::Local::now() + chrono::Duration::days(30),
//     );

//     book.set_borrow_info(Some(borrow));
//     match book.update_db(state.db_conn()).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// pub async fn return_book(state: &State, book_uuid: Uuid) -> Result<()> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Librarian => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     let mut book = match BookInfo::from_uuid_db(book_uuid, state.db_conn()).await {
//         Ok(Some(book)) => book,
//         Ok(None) => return Err(Error::BookNotFound),
//         Err(e) => return Err(Error::DatabaseError(e)),
//     };

//     book.set_borrow_info(None);
//     match book.update_db(state.db_conn()).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// // User Operations
// // User Ops
// pub async fn update_info(state: &State, new_info: UserInfo) -> Result<()> {
//     match state.login_state() {
//         Some(_) => (),
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     let user_id =
//         match UserInfo::from_phone_number_db(new_info.phone_number(), state.db_conn()).await {
//             Ok(Some(user)) => user.id(),
//             Ok(None) => return Err(Error::UserNotFound),
//             Err(e) => return Err(Error::DatabaseError(e)),
//         };

//     let mut new_info = new_info.clone();
//     new_info.set_id(user_id);

//     match new_info.update_db(state.db_conn()).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }
// pub async fn my_borrowed_books(state: &State) -> Result<Vec<BookInfo>> {
//     match state.login_state() {
//         Some(_) => (),
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     let my = state.login_state().unwrap().current_user();
//     match BookInfo::from_borrower_id_db(my.id(), state.db_conn()).await {
//         Ok(books) => Ok(books),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }
// pub async fn renew_book(state: &State, book_uuid: Uuid) -> Result<()> {
//     match state.login_state() {
//         Some(login_state) => match login_state.role() {
//             Role::Librarian => (),
//             _ => return Err(Error::NotEnoughPrivilege),
//         },
//         None => {
//             panic!("LoginState is None");
//         }
//     }

//     let mut book = match BookInfo::from_uuid_db(book_uuid, state.db_conn()).await {
//         Ok(Some(book)) => book,
//         Ok(None) => return Err(Error::BookNotFound),
//         Err(e) => return Err(Error::DatabaseError(e)),
//     };

//     let mut borrow = match book.borrow_info() {
//         Some(borrow) => borrow.clone(),
//         None => return Err(Error::BookNotBorrowed),
//     };

//     borrow.renew();
//     book.set_borrow_info(Some(borrow));

//     match book.update_db(state.db_conn()).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(Error::DatabaseError(e)),
//     }
// }

// // Book Ops
// pub fn search_book(state: &State) {
//     todo!()
// }

// TODO: use trait to abstract database operations
// pub trait Identifiable {
//     fn identifier<T>(self: &Self) -> T;
// }

// #[async_trait]
// pub trait DatabaseCrud<A, E, M, I>
// where
//     A: ActiveModelTrait,
//     E: EntityTrait,
//     M: ModelTrait,
//     I: Identifiable,
// {
//     async fn insert_db(db: &DbConn, table: &E, model: A) -> Result<()>;
//     async fn delete_db(db: DbConn, table: &E, model: A) -> Result<()>;
//     async fn update_db(db: DbConn, table: &E, model: A) -> Result<()>;
//     async fn find_db(db: DbConn, table: &E, identifier: I) -> Result<M>;
// }
