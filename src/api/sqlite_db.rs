use rocket_contrib::databases::diesel as rocket_diesel;

#[database("sqlite_db")]
pub struct SQLiteDb(rocket_diesel::SqliteConnection);


