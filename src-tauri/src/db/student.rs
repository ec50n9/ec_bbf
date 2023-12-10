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

/// 学生带分数原始结构体
#[derive(Serialize, Deserialize)]
pub struct StudentScoreRaw {
    id: String,
    name: String,
    stu_no: String,
    score_type_id: String,
    score_type_name: String,
    score_type_color: Option<String>,
    score_type_icon: Option<String>,
    score: i32,
}

/// 分数（简单结构
#[derive(Serialize, Deserialize)]
pub struct StudentScoreSimp {
    id: String,
    name: String,
    color: String,
    score: i32,
}

/// 学生结构体（带分数
#[derive(Serialize, Deserialize)]
pub struct StudentWithScore {
    id: String,
    stu_no: String,
    name: String,
    score_list: Vec<StudentScoreSimp>,
}

/// 获取全部学生（带分数
#[tauri::command]
pub fn get_student_list_with_score(
    state: tauri::State<'_, crate::AppState>,
    student_query_vo: StudentQueryVO,
) -> Result<Vec<StudentWithScore>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let StudentQueryVO { name, stu_no } = student_query_vo;

    // 字段sql
    let mut query_fields = Vec::new();
    // 参数值
    let mut params = Vec::new();

    // 添加参数
    query_fields.push("1 = 1");
    if let Some(name) = name {
        query_fields.push("s.name LIKE ?");
        params.push(format!("%{}%", name));
    }
    if let Some(stu_no) = stu_no {
        query_fields.push("s.stu_no LIKE ?");
        params.push(format!("%{}%", stu_no));
    }

    let query_sql = format!(
        "SELECT s.id                  as id,
                s.stu_no              as stu_no,
                s.name                as name,
                st.id                 as score_type_id,
                st.name               as score_type_name,
                st.color              as score_type_color,
                st.icon               as score_type_icon,
                sum(ssr.action_value) as score
        from student_score_record ssr
                left join main.score_type st on st.id = ssr.score_type_id
                left join main.student s on s.id = ssr.student_id
        where st.name not null and {}
        group by s.id, st.id;",
        query_fields.join(" AND ")
    );

    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");
    let students = stmt
        .query_map(params_from_iter(params.iter()), |row| {
            Ok(StudentScoreRaw {
                id: row.get(0)?,
                stu_no: row.get(1)?,
                name: row.get(2)?,
                score_type_id: row.get(3)?,
                score_type_name: row.get(4)?,
                score_type_color: row.get(5)?,
                score_type_icon: row.get(6)?,
                score: row.get(7)?,
            })
        })
        .expect("转换结果出错")
        .map(|item| item.unwrap())
        .collect::<Vec<StudentScoreRaw>>();

    let student_with_score_list = students
        .into_iter()
        .fold(Vec::<StudentWithScore>::new(), |mut acc, item| {
            let score = StudentScoreSimp {
                id: item.score_type_id,
                name: item.score_type_name,
                color: item.score_type_color.unwrap_or_default(),
                score: item.score,
            };

            if let Some(student) = acc.iter_mut().find(|student| student.id == item.id) {
                student.score_list.push(score);
            } else {
                acc.push(StudentWithScore {
                    id: item.id,
                    stu_no: item.stu_no,
                    name: item.name,
                    score_list: vec![score],
                });
            }

            acc
        });

    Ok(student_with_score_list)
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
