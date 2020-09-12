use rocket_contrib::databases::diesel as rocket_diesel;

#[database("sqlite_db")]
pub struct SQLiteDbCtx(rocket_diesel::SqliteConnection);

pub type SQLiteDb<'a> = &'a rocket_diesel::SqliteConnection;


