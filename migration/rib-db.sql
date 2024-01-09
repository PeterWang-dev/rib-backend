-- database: rib-db.sqlite3
PRAGMA foreign_keys = OFF;

BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS
    "seaql_migrations" (
        "version" text NOT NULL PRIMARY KEY,
        "applied_at" bigint NOT NULL
    );

INSERT INTO
    seaql_migrations
VALUES
    ('m20240109_073249_create_table', 1704802674);

CREATE TABLE IF NOT EXISTS
    "privilege" (
        "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        "name" text NOT NULL
    );

INSERT INTO
    privilege
VALUES
    (1, 'Administrator');

INSERT INTO
    privilege
VALUES
    (2, 'Librarian');

INSERT INTO
    privilege
VALUES
    (3, 'Reader');

CREATE TABLE IF NOT EXISTS
    "user" (
        "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        "password" text NOT NULL,
        "first_name" text,
        "last_name" text,
        "phone_number" text NOT NULL UNIQUE,
        "birthday" text,
        "address" text,
        "email_address" text NOT NULL UNIQUE,
        "registration_time" text NOT NULL DEFAULT CURRENT_TIME,
        "role" text NOT NULL,
        FOREIGN KEY ("role") REFERENCES "privilege" ("id") ON DELETE CASCADE ON UPDATE CASCADE
    );

CREATE TABLE IF NOT EXISTS
    "book" (
        "uuid" text (36) NOT NULL PRIMARY KEY,
        "title" text NOT NULL,
        "author" text NOT NULL,
        "publisher" text NOT NULL,
        "published_time" text NOT NULL,
        "category" text NOT NULL,
        "isbn" text NOT NULL UNIQUE,
        "borrowed_by" integer,
        "borrowed_date" text,
        "return_date" text,
        "is_renewed" boolean,
        FOREIGN KEY ("borrowed_by") REFERENCES "user" ("id") ON DELETE CASCADE ON UPDATE CASCADE
    );

DELETE FROM sqlite_sequence;

INSERT INTO
    sqlite_sequence
VALUES
    ('privilege', 3);

CREATE VIEW
    `borrow_view` (
        `user_id`,
        `book_id`,
        `book_title`,
        `borrow_date`,
        `return_date`,
        `is_renewed`
    ) AS
SELECT
    `id`,
    `uuid`,
    `title`,
    `borrowed_date`,
    `return_date`,
    `is_renewed`
FROM
    `user`
    NATURAL JOIN `book`;

COMMIT;
