use rusqlite::{params_from_iter, Connection, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 学生结构体
#[derive(Serialize, Deserialize)]
pub struct Student {
    id: String,
    stu_no: String,
    name: String,
    is_delete: bool,
}

/// 初始化数据库
pub fn init_db() -> Result<Connection> {
    let db_path = "D://db.sqlite";
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

/// 获取全部学生
#[tauri::command]
pub fn get_student_list(state: tauri::State<'_, crate::AppState>) -> Result<Vec<Student>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let mut stmt = conn
        .prepare("SELECT id, stu_no, name, is_delete FROM student WHERE is_delete = 0")
        .expect("sql预处理出错");
    let students_iter = stmt
        .query_map([], |row| {
            let is_delete = row.get::<usize, i32>(3).expect("获取 is_delete 失败") == 1;

            Ok(Student {
                id: row.get(0)?,
                stu_no: row.get(1)?,
                name: row.get(2)?,
                is_delete,
            })
        })
        .expect("转换结果出错")
        .map(|item| item.unwrap());

    Ok(students_iter.collect::<Vec<Student>>())
}

/// 根据学生id获取学生信息
#[tauri::command]
pub fn get_student_by_id(
    state: tauri::State<'_, crate::AppState>,
    id: String,
) -> Result<Student, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let mut stmt = conn
        .prepare("SELECT id, stu_no, name, is_delete FROM student WHERE id = ? and is_delete = 0")
        .expect("sql预处理出错");

    let student = stmt
        .query_row([id], |row| {
            let is_delete = row.get::<usize, i32>(3).expect("获取 is_delete 是的") == 1;

            Ok(Student {
                id: row.get(0)?,
                stu_no: row.get(1)?,
                name: row.get(2)?,
                is_delete,
            })
        })
        .expect("转换学生类型失败");
    Ok(student)
}

/// 用于创建学生的结构体
#[derive(Serialize, Deserialize)]
pub struct StudentCreateVO {
    stu_no: String,
    name: String,
}

/// 创建学生
#[tauri::command]
pub fn create_student(
    state: tauri::State<'_, crate::AppState>,
    student_create_vo: StudentCreateVO,
) -> Result<String, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let id = Uuid::new_v4().to_string();
    let StudentCreateVO { stu_no, name } = student_create_vo;
    let mut stmt = conn
        .prepare("INSERT INTO student (id, stu_no, name) VALUES (?, ?, ?)")
        .expect("sql预处理出错");
    stmt.insert([id.clone(), stu_no, name]).expect("");
    Ok(id)
}

/// 用于更新学生信息的结构体
#[derive(Serialize, Deserialize)]
pub struct StudentUpdateVO {
    id: String,
    stu_no: Option<String>,
    name: Option<String>,
}

/// 更新学生
#[tauri::command]
pub fn update_student(
    state: tauri::State<'_, crate::AppState>,
    student_update_vo: StudentUpdateVO,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let StudentUpdateVO { id, stu_no, name } = student_update_vo;

    // 字段sql
    let mut update_fields = Vec::new();
    // 参数值
    let mut params = Vec::new();

    // 添加参数
    if let Some(stu_no) = stu_no {
        update_fields.push("stu_no = ?");
        params.push(stu_no);
    }
    if let Some(name) = name {
        update_fields.push("name = ?");
        params.push(name);
    }
    params.push(id);

    // 组装sql
    let query_sql = format!(
        "UPDATE student SET {} WHERE id = ?",
        update_fields.join(", ")
    );

    // 预处理
    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");
    // 执行
    let rows = stmt
        .execute(params_from_iter(params.iter()))
        .expect("更新失败");
    Ok(rows)
}

/// 根据学生id删除学生
#[tauri::command]
pub fn delete_student(
    state: tauri::State<'_, crate::AppState>,
    id: String,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    // 组装sql
    let query_sql = format!("UPDATE student SET is_delete = 1 WHERE id = '{}'", id);

    // 预处理
    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");
    // 执行
    let rows = stmt.execute([]).expect("删除失败");
    Ok(rows)
}
