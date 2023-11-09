// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 数据库相关
mod student;
use std::sync::Mutex;
use student::init_db;

// 窗口阴影
mod utils;
use crate::utils::set_window_shadow;

/** 应用状态 */
pub struct AppState {
    db_conn: Mutex<rusqlite::Connection>,
}

/** 入口函数 */
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .invoke_handler(
            register_handlers![student::{create_student, batch_create_student, get_student_list, get_student_by_id, update_student, delete_student}],
        )
        .manage(AppState {
            db_conn: Mutex::new(init_db().unwrap()),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
