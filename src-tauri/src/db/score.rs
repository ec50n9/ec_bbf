use rusqlite::{params_from_iter, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 分数类型结构体
#[derive(Serialize, Deserialize)]
pub struct ScoreType {
    id: String,
    name: String,
    desc: String,
    max: i32,
}

/// 用于创建分数类型的结构体
#[derive(Serialize, Deserialize)]
pub struct ScoreTypeCreateVO {
    name: String,
    desc: String,
    max: i32,
}

/// 创建分数类型
#[tauri::command]
pub fn create_score_type(
    state: tauri::State<'_, crate::AppState>,
    score_type_create_vo: ScoreTypeCreateVO,
) -> Result<String, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let id = Uuid::new_v4().to_string();
    let ScoreTypeCreateVO { name, desc, max } = score_type_create_vo;
    let mut stmt = conn
        .prepare("INSERT INTO score_type (id, name, desc, max) VALUES (?, ?, ?, ?)")
        .expect("sql预处理出错");
    stmt.insert([id.clone(), name, desc, max.to_string()])
        .expect("");
    Ok(id)
}

/// 用于更新分数类型的结构体
#[derive(Serialize, Deserialize)]
pub struct ScoreTypeUpdateVO {
    id: String,
    name: Option<String>,
    desc: Option<String>,
    max: Option<i32>,
}

/// 更新学生来源类型
#[tauri::command]
pub fn update_score_type(
    state: tauri::State<'_, crate::AppState>,
    score_type_update_vo: ScoreTypeUpdateVO,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let ScoreTypeUpdateVO {
        id,
        name,
        desc,
        max,
    } = score_type_update_vo;

    // 字段sql
    let mut update_fields = Vec::new();
    // 参数值
    let mut params = Vec::new();

    // 添加参数
    if let Some(name) = name {
        update_fields.push("name = ?");
        params.push(name);
    }
    if let Some(desc) = desc {
        update_fields.push("desc = ?");
        params.push(desc);
    }
    if let Some(max) = max {
        update_fields.push("max = ?");
        params.push(max.to_string());
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

/// 删除分数类型
#[tauri::command]
pub fn delete_score_type(
    state: tauri::State<'_, crate::AppState>,
    id: usize,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    // 预处理
    let mut stmt = conn
        .prepare("DELETE FROM score_type WHERE id = ?")
        .expect("sql预处理出错");
    // 执行
    let rows = stmt.execute([id]).expect("删除失败");
    Ok(rows)
}

/// 加分入参
#[derive(Serialize, Deserialize)]
pub struct ScorePlusVO {
    student_id: String,
    score_type_id: String,
    score: i32,
}

/// 根据学生id和分数类型id为学生加分（可为负数
#[tauri::command]
pub fn add_score(
    state: tauri::State<'_, crate::AppState>,
    score_plus_vo: ScorePlusVO,
) -> Result<(), String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let ScorePlusVO {
        student_id,
        score_type_id,
        score,
    } = score_plus_vo;

    let query_sql =
        "SELECT score FROM student_score_mapping WHERE student_id = ? AND score_type_id = ?";
    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");
    let origin_score_result = stmt.query_row([&student_id, &score_type_id], |row| {
        Ok(row.get::<usize, i32>(0)?)
    });

    match origin_score_result {
        Ok(origin_score) => {
            let new_score: i32 = origin_score + score;
            let mut stmt = conn
                .prepare("UPDATE student_score_mapping SET score = ? WHERE student_id = ? AND score_type_id = ?")
                .expect("sql预处理出错");
            stmt.execute([new_score.to_string(), student_id, score_type_id])
                .expect("更新失败");
        }
        Err(_) => {
            let mut stmt = conn
                .prepare("INSERT INTO student_score_mapping (student_id, score_type_id, score) VALUES (?, ?, ?)")
                .expect("sql预处理出错");
            stmt.insert([student_id, score_type_id, score.to_string()])
                .expect("插入失败");
        }
    }

    Ok(())
}

/// 分数查询出参
#[derive(Serialize, Deserialize)]
pub struct Score {
    score_type_id: String,
    score_type_name: String,
    score_type_desc: String,
    max: i32,
    score: i32,
}

/// 根据学生id查询学生的所有分数
#[tauri::command]
pub fn get_score_list_by_student_id(
    state: tauri::State<'_, crate::AppState>,
    student_id: String,
) -> Result<Vec<Score>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let query_sql = "SELECT * FROM student_score_mapping WHERE student_id = ?";
    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");

    let score_list = stmt
        .query_map([student_id], |row| {
            Ok(Score {
                score_type_id: row.get(0)?,
                score_type_name: row.get(1)?,
                score_type_desc: row.get(2)?,
                max: row.get(3)?,
                score: row.get(4)?,
            })
        })
        .expect("查询失败")
        .map(|item| item.unwrap())
        .collect::<Vec<Score>>();
    Ok(score_list)
}
