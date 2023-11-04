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

/** 自定义handler注册器，可以批量注册 */
macro_rules! register_handlers {
    ($( $module:ident :: { $( $handler:ident ),* } ),*) => {
        tauri::generate_handler![
            $(
                $( $module::$handler ),*
            ),*
        ]
    };
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .invoke_handler(
            register_handlers![student::{create_student, get_student_list, update_student, delete_student}],
        )
        .manage(AppState {
            db_conn: Mutex::new(init_db().unwrap()),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
