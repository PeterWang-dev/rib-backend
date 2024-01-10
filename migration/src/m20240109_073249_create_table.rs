use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_query::Table;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Get the database backend
        let db = manager.get_connection();
        let builder = db.get_database_backend();

        // Privilege Table
        manager
            .create_table(
                Table::create()
                    .table(Privilege::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Privilege::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Privilege::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // User Table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string())
                    .col(ColumnDef::new(User::LastName).string())
                    .col(ColumnDef::new(User::Birthday).date())
                    .col(ColumnDef::new(User::Address).string())
                    .col(
                        ColumnDef::new(User::PhoneNumber)
                            .string()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::EmailAddress)
                            .string()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::RegistrationTime)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(User::RoleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_user_privilege")
                            .from(User::Table, User::RoleId)
                            .to(Privilege::Table, Privilege::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Book Table
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Book::Uuid).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(ColumnDef::new(Book::Author).string().not_null())
                    .col(ColumnDef::new(Book::Publisher).string().not_null())
                    .col(ColumnDef::new(Book::PublishedTime).date_time().not_null())
                    .col(ColumnDef::new(Book::Category).string().not_null())
                    .col(ColumnDef::new(Book::Isbn).string().not_null().unique_key())
                    .col(ColumnDef::new(Book::BorrowedBy).integer())
                    .col(ColumnDef::new(Book::BorrowedDate).date_time())
                    .col(ColumnDef::new(Book::ReturnDate).date_time())
                    .col(ColumnDef::new(Book::IsRenewed).boolean().not_null().default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_book_user")
                            .from(Book::Table, Book::BorrowedBy)
                            .to(User::Table, User::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Insert Privileges
        let insert_privileges_stmt = sea_query::Query::insert()
            .into_table(Privilege::Table)
            .columns(vec![Privilege::Id, Privilege::Name])
            .values_panic([1.into(), "Administrator".into()])
            .values_panic([2.into(), "Librarian".into()])
            .values_panic([3.into(), "Reader".into()])
            .to_owned();
        db.execute(builder.build(&insert_privileges_stmt)).await?;

        // Borrow View using almost raw SQL
        let create_view_stmt = Statement::from_sql_and_values(
            builder,
            r#"
            CREATE VIEW
                "borrow_view" (
                    "user_id",
                    "book_uuid",
                    "book_title",
                    "borrow_date",
                    "return_date",
                    "is_renewed"
                ) AS
            SELECT
                "id",
                "uuid",
                "title",
                "borrowed_date",
                "return_date",
                "is_renewed"
            FROM
                "user",
                "book"
            WHERE
                "id" = "borrowed_by";"#,
            [],
        );

        db.execute(create_view_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Get the database backend
        let db = manager.get_connection();
        let builder = db.get_database_backend();

        // Drop View
        let drop_view_stmt =
            Statement::from_sql_and_values(builder, r#"DROP VIEW borrow_view;"#, []);
        db.execute(drop_view_stmt).await?;

        // Drop Table
        manager
            .drop_table(Table::drop().table(Privilege::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Privilege {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Password,
    FirstName,
    LastName,
    PhoneNumber,
    Birthday,
    Address,
    EmailAddress,
    RegistrationTime,
    RoleId,
}

// #[derive(DeriveIden)]
// enum Stuff {
//     Table,
//     UserId,
//     PrivilegeId,
//     EntryDate,
//     BankAccount,
// }

// #[derive(DeriveIden)]
// enum Reader {
//     Table,
//     UserId,
//     RegistrationTime,
// }

#[derive(DeriveIden)]
enum Book {
    Table,
    Uuid,
    Title,
    Author,
    Publisher,
    PublishedTime,
    Category,
    Isbn,
    // ShelfInfo,
    BorrowedBy,
    BorrowedDate,
    ReturnDate,
    IsRenewed,
}
