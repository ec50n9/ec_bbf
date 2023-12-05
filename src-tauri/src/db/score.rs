use rusqlite::{params_from_iter, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 分数类型结构体
#[derive(Serialize, Deserialize)]
pub struct ScoreType {
    id: String,
    name: String,
    desc: String,
    icon: Option<String>,
    color: Option<String>,
}

/// 获取全部分数类型
#[tauri::command]
pub fn get_score_type_list(
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<ScoreType>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");
    let query_sql = "SELECT id, name, desc, icon, color FROM score_type";

    let mut stmt = conn.prepare(query_sql).expect("sql预处理出错");
    let score_iter = stmt
        .query_map([], |row| {
            Ok(ScoreType {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
                icon: row.get(3)?,
                color: row.get(4)?,
            })
        })
        .expect("转换结果出错")
        .map(|item| item.unwrap());

    Ok(score_iter.collect())
}

/// 获取单个分数类型
#[tauri::command]
pub fn get_score_type_by_id(
    state: tauri::State<'_, crate::AppState>,
    id: String,
) -> Result<ScoreType, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");
    let query_sql = "SELECT id, name, desc, icon, color FROM score_type WHERE id = ?";
    let mut stmt = conn.prepare(query_sql).expect("sql预处理出错");
    let score_type = stmt
        .query_row([id], |row| {
            Ok(ScoreType {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
                icon: row.get(3)?,
                color: row.get(4)?,
            })
        })
        .expect("转换结果出错");

    Ok(score_type)
}

/// 用于创建分数类型的结构体
#[derive(Serialize, Deserialize)]
pub struct ScoreTypeCreateVO {
    name: String,
    desc: String,
    icon: Option<String>,
    color: Option<String>,
}

/// 创建分数类型
#[tauri::command]
pub fn create_score_type(
    state: tauri::State<'_, crate::AppState>,
    score_type_create_vo: ScoreTypeCreateVO,
) -> Result<String, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let id = Uuid::new_v4().to_string();
    let ScoreTypeCreateVO {
        name,
        desc,
        icon,
        color,
    } = score_type_create_vo;

    let mut insert_fields = vec!["id", "name", "desc"];
    let mut params = vec![id.clone(), name, desc];

    if let Some(icon) = icon {
        insert_fields.push("icon");
        params.push(icon);
    }
    if let Some(color) = color {
        insert_fields.push("color");
        params.push(color);
    }

    let sql = format!(
        "INSERT INTO score_type ({}) VALUES ({})",
        insert_fields.join(","),
        std::iter::repeat("?")
            .take(params.len())
            .collect::<Vec<_>>()
            .join(",")
    );
    let mut stmt = conn.prepare(&sql).expect("sql预处理出错");
    stmt.insert(params_from_iter(params.iter()))
        .expect("插入数据失败");
    Ok(id)
}

/// 用于更新分数类型的结构体
#[derive(Serialize, Deserialize)]
pub struct ScoreTypeUpdateVO {
    id: String,
    name: Option<String>,
    desc: Option<String>,
    icon: Option<String>,
    color: Option<String>,
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
        icon,
        color,
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
    if let Some(icon) = icon {
        update_fields.push("icon = ?");
        params.push(icon);
    }
    if let Some(color) = color {
        update_fields.push("color = ?");
        params.push(color);
    }
    params.push(id);

    // 组装sql
    let query_sql = format!(
        "UPDATE score_type SET {} WHERE id = ?",
        update_fields.join(", ")
    );

    // 预处理
    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");
    // 执行
    let rows = stmt.execute(params_from_iter(params)).expect("更新失败");
    Ok(rows)
}

/// 删除分数类型
#[tauri::command]
pub fn delete_score_type(
    state: tauri::State<'_, crate::AppState>,
    id: String,
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
    action_value: i32,
    reason: Option<String>,
}

/// 根据学生id和分数类型id为学生加分（可为负数
#[tauri::command]
pub fn add_score(
    state: tauri::State<'_, crate::AppState>,
    score_plus_vo: ScorePlusVO,
) -> Result<(), String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let id = Uuid::new_v4().to_string();
    let ScorePlusVO {
        student_id,
        score_type_id,
        action_value,
        reason,
    } = score_plus_vo;
    let mut stmt = conn
        .prepare(
            "INSERT INTO student_score_record (id, student_id, score_type_id, action_value, reason) VALUES (?, ?, ?, ?, ?)",
        )
        .expect("sql预处理出错");
    stmt.insert([
        id,
        student_id,
        score_type_id,
        action_value.to_string(),
        reason.unwrap_or(String::new()),
    ])
    .expect("插入失败");

    Ok(())
}

/// 分数查询出参
#[derive(Serialize, Deserialize)]
pub struct Score {
    score_type_id: String,
    score: i32,
}

/// 根据学生id查询学生的所有分数
#[tauri::command]
pub fn get_score_list_by_student_id(
    state: tauri::State<'_, crate::AppState>,
    student_id: String,
) -> Result<Vec<Score>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let query_sql = "SELECT score_type_id, SUM(action_value) AS score FROM student_score_record WHERE student_id = ? GROUP BY score_type_id";
    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");

    let score_list = stmt
        .query_map([student_id], |row| {
            Ok(Score {
                score_type_id: row.get(0)?,
                score: row.get(1)?,
            })
        })
        .expect("查询失败")
        .map(|item| item.unwrap())
        .collect::<Vec<Score>>();
    Ok(score_list)
}

/// 每日分数统计出参
#[derive(Serialize, Deserialize)]
pub struct ScoreStatistics {
    date: String,
    positive_count: i32,
    negative_count: i32,
    positive_total_score: i32,
    negative_total_score: i32,
}

/// 根据学生id查询指定时间段内的每日分数统计
/// 日期示例：2022-01-01
#[tauri::command]
pub fn get_daily_score_by_student_id(
    state: tauri::State<'_, crate::AppState>,
    student_id: String,
    start_date: String,
    end_date: String,
) -> Result<Vec<ScoreStatistics>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let query_sql = "SELECT
            DATE(create_time)                                      AS date,
            COUNT(CASE WHEN action_value >= 0 THEN 1 END)          AS positive_count,
            COUNT(CASE WHEN action_value < 0 THEN 1 END)           AS negative_count,
            COALESCE(SUM(CASE WHEN action_value >= 0 THEN action_value END), 0) AS positive_total_score,
            COALESCE(SUM(CASE WHEN action_value < 0 THEN action_value END), 0)  AS negative_total_score
        FROM
            student_score_record
        WHERE
            student_id = ?
            AND create_time >= ?
            AND create_time < ?
        GROUP BY DATE(create_time);";
    let mut stmt = conn.prepare(&query_sql).expect("sql预处理出错");

    let score_list = stmt
        .query_map([student_id, start_date, end_date], |row| {
            Ok(ScoreStatistics {
                date: row.get(0)?,
                positive_count: row.get(1)?,
                negative_count: row.get(2)?,
                positive_total_score: row.get(3)?,
                negative_total_score: row.get(4)?,
            })
        })
        .expect("查询失败")
        .map(|item| item.unwrap())
        .collect::<Vec<ScoreStatistics>>();
    Ok(score_list)
}
