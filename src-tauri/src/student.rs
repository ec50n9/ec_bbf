use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub stu_no: String,
    pub name: String,
    pub is_delete: bool,
}

pub fn init_db() -> Result<Connection> {
    let db_path = "db.sqlite";
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS student (
              id          varchar(64)     PRIMARY KEY,
              stu_no      varchar(20)     NOT NULL,
              name        varchar(20)     NOT NULL,
              is_delete   numeric         DEFAULT 0
            )",
        [],
    )?;
    Ok(conn)
}

#[tauri::command]
pub fn get_list(state: tauri::State<'_, crate::AppState>) -> Vec<Student> {
    let conn = state.db_conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM student").unwrap();
    let students_iter = stmt
        .query_map([], |row| {
            let is_delete = row.get::<usize, i32>(3).unwrap() == 1;

            Ok(Student {
                id: row.get(0)?,
                stu_no: row.get(1)?,
                name: row.get(2)?,
                is_delete,
            })
        })
        .unwrap()
        .map(|item| item.unwrap());

    students_iter.collect::<Vec<Student>>()
}

#[tauri::command]
pub fn create(state: tauri::State<'_, crate::AppState>, student: Student) -> bool {
    let conn = state.db_conn.lock().unwrap();

    let Student {
        id, stu_no, name, ..
    } = student;
    match conn.execute(
        "INSERT INTO student (id, stu_no, name) VALUES (?, ?, ?)",
        [id, stu_no, name],
    ) {
        Ok(insert) => {
            println!("{} row inserted", insert);
            true
        }
        Err(err) => {
            println!("some error: {}", err);
            false
        }
    }
}
