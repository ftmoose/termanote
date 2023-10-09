use rusqlite::Connection;
pub const DBNAME: &str = "termanote.db";

pub fn connect_to(name: &str) -> Connection {
  rusqlite::Connection::open(name)
      .expect("Failed to open database")
}

pub fn connect() -> Connection {
  connect_to(DBNAME)
}
