// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use crate::utils::set_window_shadow;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// sqlite相关
mod student;
use rusqlite::Connection;
use std::sync::Mutex;
use student::init_db;

pub struct AppState {
    db_conn: Mutex<Connection>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            student::get_list,
            student::create
        ])
        .manage(AppState {
            db_conn: Mutex::new(init_db().unwrap()),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
