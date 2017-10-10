#![deny(warnings)]
//! # Sqlite support for the `r2d2` connection pool.
//!
//! Library crate: [r2d2-sqlite](https://crates.io/crates/r2d2-sqlite/)
//!
//! Integrated with: [r2d2](https://crates.io/crates/r2d2)
//! and [rusqlite](https://crates.io/crates/rusqlite)
//!
//! ## Example
//!
//! ```rust,no_run
//! extern crate r2d2;
//! extern crate r2d2_sqlite3;
//! extern crate sqlite;
//!
//! use std::thread;
//! use r2d2_sqlite3::SqliteConnectionManager;
//!
//! fn main() {
//!     let config = r2d2::Config::default();
//!     let manager = SqliteConnectionManager::file("file.db");
//!     let pool = r2d2::Pool::new(config, manager).unwrap();
//!
//!     for i in 0..10i32 {
//!         let pool = pool.clone();
//!         thread::spawn(move || {
//!             let conn = pool.get().unwrap();
//!             let mut stmt = conn.prepare("INSERT INTO foo (bar) VALUES (?)").unwrap();
//!             stmt.bind(1, 42).unwrap();
//!         });
//!     }
//! }
//! ```
extern crate r2d2;
extern crate sqlite;


use sqlite::{Connection, Error};
use std::path::{Path,PathBuf};



enum ConnectionConfig {
    File(PathBuf),
    Memory,
}

/// An `r2d2::ManageConnection` for `rusqlite::Connection`s.
pub struct SqliteConnectionManager(ConnectionConfig);

impl SqliteConnectionManager {
    /// Creates a new `SqliteConnectionManager` from file.
    ///
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        SqliteConnectionManager(
            ConnectionConfig::File(path.as_ref().to_path_buf()))
    }

    pub fn memory() -> Self {
        SqliteConnectionManager(ConnectionConfig::Memory)
    }

}

impl r2d2::ManageConnection for SqliteConnectionManager {
    type Connection = Connection;
    type Error = sqlite::Error;

    fn connect(&self) -> Result<Connection, Error> {
        match self.0{
            ConnectionConfig::File(ref path) => Connection::open(path),
            ConnectionConfig::Memory => Connection::open(":memory:")
        }
    }

    fn is_valid(&self, conn: &mut Connection) -> Result<(), Error> {
        conn.execute("").map_err(Into::into)
    }

    fn has_broken(&self, _: &mut Connection) -> bool {
        false
    }
}
