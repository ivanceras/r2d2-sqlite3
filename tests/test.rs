extern crate r2d2;
extern crate r2d2_sqlite3;
extern crate sqlite3;
extern crate tempdir;

use std::sync::mpsc;
use std::thread;

use r2d2::ManageConnection;
use tempdir::TempDir;

use r2d2_sqlite3::SqliteConnectionManager;
use sqlite3::Connection;

#[test]
fn test_basic() {
    let manager = SqliteConnectionManager::file("file.db");
    let pool = r2d2::Pool::builder().max_size(2).build(manager).unwrap();

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = thread::spawn(move || {
        let conn = pool1.get().unwrap();
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn);
    });

    let pool2 = pool.clone();
    let t2 = thread::spawn(move || {
        let conn = pool2.get().unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    pool.get().unwrap();
}

#[test]
fn test_file() {
    let manager = SqliteConnectionManager::file("file.db");
    let pool = r2d2::Pool::builder().max_size(2).build(manager).unwrap();

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = thread::spawn(move || {
        let conn = pool1.get().unwrap();
        let conn1: &Connection = &*conn;
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn1);
    });

    let pool2 = pool.clone();
    let t2 = thread::spawn(move || {
        let conn = pool2.get().unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    pool.get().unwrap();
}

#[test]
fn test_is_valid() {
    let manager = SqliteConnectionManager::file("file.db");
    let pool = r2d2::Pool::builder()
        .max_size(1)
        .test_on_check_out(true)
        .build(manager)
        .unwrap();

    pool.get().unwrap();
}

#[test]
fn test_error_handling() {
    //! We specify a directory as a database. This is bound to fail.
    let dir = TempDir::new("r2d2-sqlite").expect("Could not create temporary directory");
    let dirpath = dir.path().to_str().unwrap();
    let manager = SqliteConnectionManager::file(dirpath);
    assert!(manager.connect().is_err());
}
