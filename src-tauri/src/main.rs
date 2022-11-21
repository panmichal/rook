#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use keyring;
use rusqlite::Connection;
use serde::Serialize;
use std::{ops::Deref, sync::Mutex};
use tauri::State;

#[derive(Serialize)]
struct Participant {
    id: u32,
    name: String,
    description: Option<String>,
}

#[derive(Clone)]
struct AppSettings {
    encrypted_db: bool,
}

struct DbConnection {
    db: Mutex<Option<Connection>>,
}

impl AppSettings {
    fn new() -> AppSettings {
        AppSettings {
            encrypted_db: false,
        }
    }

    fn set_encrypted(&mut self, encrypted: bool) -> AppSettings {
        self.encrypted_db = encrypted;
        self.deref().clone()
    }

    fn encrypted(&self) -> bool {
        self.encrypted_db
    }
}

#[tauri::command]
fn people_list(conn: State<DbConnection>) -> Vec<Participant> {
    let c = conn.db.lock().unwrap();
    let mut select = c
        .as_ref()
        .unwrap()
        .prepare("SELECT id, name, description FROM participants")
        .unwrap();

    select
        .query_map([], |row| {
            Ok(Participant {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })
        .unwrap()
        .map(|p| p.unwrap())
        .collect()
}

fn main() {
    let settings = AppSettings::new().set_encrypted(false);

    //ignore password handling for now
    if settings.encrypted() {
        let service = app_key();
        let username = "db";
        let entry = keyring::Entry::new(&service, &username);
        match entry.get_password() {
            Ok(pass) => pass,
            Err(_err) => {
                println!("No pass");
                let password = "secret";
                entry.set_password(&password).unwrap();
                password.to_owned()
            }
        };
    }

    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE participants (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            description  TEXT
        )",
        (), // empty list of parameters.
    )
    .expect("Could not create database table");

    let p = Participant {
        id: 0,
        name: "Corvus frugilegus".to_owned(),
        description: None,
    };

    conn.execute(
        "INSERT INTO participants (name, description) VALUES (?1, ?2)",
        (&p.name, &p.description),
    )
    .expect("Could not insert test data");

    tauri::Builder::default()
        .manage(DbConnection {
            db: Mutex::new(Some(conn)),
        })
        .invoke_handler(tauri::generate_handler![people_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn app_key() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("rook_{}", version)
}
