// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

// 数据库相关
mod db;
use db::init_db;
use db::score;
use db::student;

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
        .invoke_handler(register_handlers![
            student::{
                create_student,
                batch_create_student,
                get_student_list,
                get_student_by_id,
                update_student,
                delete_student
            }, score::{
                get_score_type_list,
                get_score_type_by_id,
                create_score_type,
                update_score_type,
                delete_score_type,
                add_score,
                get_score_list_by_student_id,
                get_daily_score_by_student_id
            }
        ])
        .manage(AppState {
            db_conn: Mutex::new(init_db().unwrap()),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
