#[allow(unused_imports)]
use diesel::{
    r2d2::{self, ConnectionManager},
    sql_query,
    sqlite::SqliteConnection,
};

embed_migrations!();

pub type Connection = SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

#[cfg(test)]
pub fn migrate_and_config_db(url: &str) -> Pool {
    info!("Migrating and configuring database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    embedded_migrations::run(&pool.get().expect("Failed to migrate."));

    pool
}

#[cfg(not(test))]
pub fn migrate_and_config_db(url: &str) -> Pool {
    use crate::diesel::RunQueryDsl;
    info!("Migrating and configuring database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    sql_query(r#"DROP TABLE IF EXISTS login_history;"#).execute(&pool.get().unwrap());
    sql_query(r#"DROP TABLE IF EXISTS users;"#).execute(&pool.get().unwrap());
    sql_query(
        r#"CREATE TABLE users (
        id INTEGER PRIMARY KEY NOT NULL,
        username TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        login_session TEXT NOT NULL DEFAULT ''
    );"#,
    )
    .execute(&pool.get().unwrap());
    sql_query(
        r#"CREATE TABLE login_history (
        id INTEGER PRIMARY KEY NOT NULL,
        user_id INTEGER NOT NULL REFERENCES users(id),
        login_timestamp INTEGER NOT NULL
    );"#,
    )
    .execute(&pool.get().unwrap());
    sql_query(
        r#"CREATE TABLE schematics (
        id TEXT PRIMARY KEY NOT NULL,
        title TEXT NOT NULL,
        description TEXT NOT NULL,
        author INTEGER NOT NULL REFERENCES users(id),
        tags TEXT NOT NULL,
        display TEXT NOT NULL,
        date INTEGER NOT NULL
    );"#,
    )
    .execute(&pool.get().unwrap());

    pool
}
