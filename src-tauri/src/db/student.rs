use rusqlite::{params_from_iter, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 学生结构体
#[derive(Serialize, Deserialize)]
pub struct Student {
    id: String,
    stu_no: String,
    name: String,
}

/// 学生列表搜索参数
#[derive(Serialize, Deserialize)]
pub struct StudentQueryVO {
    name: Option<String>,
    stu_no: Option<String>,
}

/// 获取全部学生
#[tauri::command]
pub fn get_student_list(
    state: tauri::State<'_, crate::AppState>,
    student_query_vo: StudentQueryVO,
) -> Result<Vec<Student>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let StudentQueryVO { name, stu_no } = student_query_vo;

    // 字段sql
    let mut query_fields = Vec::new();
    // 参数值
    let mut params = Vec::new();

    // 添加参数
    query_fields.push("1 = 1");
    if let Some(name) = name {
        query_fields.push("name LIKE ?");
        params.push(format!("%{}%", name));
    }
    if let Some(stu_no) = stu_no {
        query_fields.push("stu_no LIKE ?");
        params.push(format!("%{}%", stu_no));
    }

    // 组装sql
    let query_sql = format!(
        "SELECT id, stu_no, name FROM student WHERE {}",
        query_fields.join(" AND ")
    );

    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");
    let students_iter = stmt
        .query_map(params_from_iter(params.iter()), |row| {
            Ok(Student {
                id: row.get(0)?,
                stu_no: row.get(1)?,
                name: row.get(2)?,
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
        .prepare("SELECT id, stu_no, name FROM student WHERE id = ?")
        .expect("sql预处理出错");

    let student = stmt
        .query_row([id], |row| {
            Ok(Student {
                id: row.get(0)?,
                stu_no: row.get(1)?,
                name: row.get(2)?,
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

/// 批量创建学生
#[tauri::command]
pub fn batch_create_student(
    state: tauri::State<'_, crate::AppState>,
    student_create_vos: Vec<StudentCreateVO>,
) -> Result<Vec<String>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let mut ids = Vec::new();
    for student_create_vo in student_create_vos {
        let id = Uuid::new_v4().to_string();
        let StudentCreateVO { stu_no, name } = student_create_vo;
        let mut stmt = conn
            .prepare("INSERT INTO student (id, stu_no, name) VALUES (?, ?, ?)")
            .expect("sql预处理出错");
        stmt.insert([id.clone(), stu_no, name]).expect("");
        ids.push(id);
    }

    Ok(ids)
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

    // 预处理
    let mut stmt = conn
        .prepare("DELETE FROM student WHERE id = ?")
        .expect("sql预处理出错");
    // 执行
    let rows = stmt.execute([id]).expect("删除失败");
    Ok(rows)
}
