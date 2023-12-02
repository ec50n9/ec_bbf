use rusqlite::{params_from_iter, OptionalExtension, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 学生分组结构体
#[derive(Serialize, Deserialize)]
pub struct StudentGroup {
    id: String,
    name: String,
    desc: String,
}

/// 获取全部学生分组
#[tauri::command]
pub fn get_student_group_list(
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<StudentGroup>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");
    let query_sql = "SELECT id, name, desc FROM student_group";

    let mut stmt = conn.prepare(query_sql).expect("sql预处理出错");
    let student_group_iter = stmt
        .query_map([], |row| {
            Ok(StudentGroup {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
            })
        })
        .expect("转换结果出错")
        .map(|item| item.unwrap());

    Ok(student_group_iter.collect())
}

/// 根据分组id获取学生分组
#[tauri::command]
pub fn get_student_group_by_id(
    state: tauri::State<'_, crate::AppState>,
    id: String,
) -> Result<StudentGroup, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let query_sql = "SELECT id, name, desc FROM student_group WHERE id = ?";
    let mut stmt = conn.prepare(query_sql).expect("sql预处理出错");
    let student_group = stmt
        .query_row([id], |row| {
            Ok(StudentGroup {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
            })
        })
        .expect("转换结果出错");

    Ok(student_group)
}

#[derive(Serialize, Deserialize)]
pub struct StudentGroupCreateVO {
    name: String,
    desc: String,
}

/// 创建学生分组
#[tauri::command]
pub fn create_student_group(
    state: tauri::State<'_, crate::AppState>,
    student_group_create_vo: StudentGroupCreateVO,
) -> Result<String, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let id = Uuid::new_v4().to_string();
    let StudentGroupCreateVO { name, desc } = student_group_create_vo;
    let mut stmt = conn
        .prepare("INSERT INTO student_group (id, name, desc) VALUES (?, ?, ?)")
        .expect("sql预处理出错");
    stmt.insert([id.clone(), name, desc]).expect("插入数据失败");
    Ok(id)
}

#[derive(Serialize, Deserialize)]
pub struct StudentGroupUpdateVO {
    id: String,
    name: Option<String>,
    desc: Option<String>,
}

/// 更新学生分组
#[tauri::command]
pub fn update_student_group(
    state: tauri::State<'_, crate::AppState>,
    student_group_update_vo: StudentGroupUpdateVO,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let StudentGroupUpdateVO { id, name, desc } = student_group_update_vo;

    let mut update_fields = Vec::new();
    let mut params = Vec::new();

    if let Some(name) = name {
        update_fields.push("name = ?");
        params.push(name);
    }
    if let Some(desc) = desc {
        update_fields.push("desc = ?");
        params.push(desc);
    }
    params.push(id);

    // 组装sql
    let sql = format!(
        "UPDATE student_group SET {} WHERE id = ?",
        update_fields.join(",")
    );

    // 预处理
    let mut stmt = conn.prepare(&sql).expect("sql预处理出错");
    let rows = stmt
        .execute(params_from_iter(params))
        .expect("更新数据失败");

    Ok(rows)
}

/// 删除学生分组
#[tauri::command]
pub fn delete_student_group(
    state: tauri::State<'_, crate::AppState>,
    id: String,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    let mut stmt = conn
        .prepare("DELETE FROM student_group WHERE id = ?")
        .expect("sql预处理出错");
    let rows = stmt.execute([id]).expect("删除数据失败");
    Ok(rows)
}

/// 学生和分组绑定的入参
#[derive(Serialize, Deserialize)]
pub struct StudentGroupBindVO {
    student_id: String,
    group_id: String,
}

/// 学生和分组解绑的入参
#[derive(Serialize, Deserialize)]
pub struct StudentGroupUnbindVO {
    student_id: String,
}

/// 学生和分组关系更新的入参
#[derive(Serialize, Deserialize)]
pub struct StudentGroupUpdateRelVO {
    student_id: String,
    update_to_group_id: String,
}

/// 批量更新学生分组绑定
#[tauri::command]
pub fn batch_update_student_group_rel(
    state: tauri::State<'_, crate::AppState>,
    need_bind_list: Vec<StudentGroupBindVO>,
    need_unbind_list: Vec<StudentGroupUnbindVO>,
    need_update_rel_list: Vec<StudentGroupUpdateRelVO>,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    // 插入
    let mut insert_stmt = conn
        .prepare("INSERT INTO student_group_mapping (student_id, group_id) VALUES (?, ?)")
        .expect("sql预处理出错");
    for item in need_bind_list {
        insert_stmt
            .execute(params_from_iter([&item.student_id, &item.group_id]))
            .expect("插入数据失败");
    }

    // 删除
    let mut delete_stmt = conn
        .prepare("DELETE FROM student_group_mapping WHERE student_id = ?")
        .expect("sql预处理出错");
    for item in need_unbind_list {
        delete_stmt
            .execute(params_from_iter([&item.student_id]))
            .expect("删除数据失败");
    }

    // 更新
    let mut update_stmt = conn
        .prepare("UPDATE student_group_mapping SET group_id = ? WHERE student_id = ?")
        .expect("sql预处理出错");
    for item in need_update_rel_list {
        update_stmt
            .execute(params_from_iter([
                &item.update_to_group_id,
                &item.student_id,
            ]))
            .expect("更新数据失败");
    }

    Ok(0)
}

/// 绑定学生和分组
#[tauri::command]
pub fn bind_student_group(
    state: tauri::State<'_, crate::AppState>,
    student_id: String,
    group_id: String,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    // 检查是否已经绑定
    let origin_group_id: Option<String> = conn
        .query_row(
            "SELECT group_id FROM student_group_mapping WHERE student_id = ?",
            [&student_id, &group_id],
            |row| row.get(0).optional(),
        )
        .expect("查询数据失败");

    // 判断是否已经绑定，如果已经绑定，并且origin_group_id = group_id，则直接返回
    if let Some(origin_group_id) = origin_group_id {
        if origin_group_id != group_id {
            let mut stmt = conn
                .prepare("UPDATE student_group_mapping SET group_id = ? WHERE student_id = ?")
                .expect("sql预处理出错");
            // 更新
            let rows = stmt
                .execute([&group_id, &student_id])
                .expect("更新数据失败");
            Ok(rows)
        } else {
            Ok(0)
        }
    } else {
        // 插入
        let mut stmt = conn
            .prepare("INSERT INTO student_group_mapping (student_id, group_id) VALUES (?, ?)")
            .expect("sql预处理出错");
        let rows = stmt.insert([&student_id, &group_id]).expect("插入数据失败") as usize;
        Ok(rows)
    }
}

/// 解绑学生和分组
#[tauri::command]
pub fn unbind_student_group(
    state: tauri::State<'_, crate::AppState>,
    student_id: String,
    group_id: String,
) -> Result<usize, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");

    // 检查是否已经绑定
    let origin_group_id: Option<String> = conn
        .query_row(
            "SELECT group_id FROM student_group_mapping WHERE student_id = ?",
            [&student_id, &group_id],
            |row| row.get(0).optional(),
        )
        .expect("查询数据失败");

    if let Some(origin_group_id) = origin_group_id {
        if origin_group_id == group_id {
            let mut stmt = conn
                .prepare("DELETE FROM student_group_mapping WHERE student_id = ?")
                .expect("sql预处理出错");
            // 删除
            let rows = stmt.execute([&student_id]).expect("删除数据失败");
            Ok(rows)
        } else {
            Ok(0)
        }
    } else {
        Ok(0)
    }
}

#[derive(Serialize, Deserialize)]
pub struct StudentWithGroupIdVO {
    group_id: Option<String>,
    student_id: String,
    student_name: String,
    student_no: String,
}

/// 获取全部学生绑定关系
#[tauri::command]
pub fn get_all_student_group_mapping(
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<StudentWithGroupIdVO>, String> {
    let conn = state.db_conn.lock().expect("获取数据库连接失败");
    let query_sql = "SELECT 
                                t2.group_id as group_id, 
                                t1.id as student_id, 
                                t1.name as student_name, 
                                t1.stu_no as student_no 
                            FROM student t1 
                            LEFT JOIN student_group_mapping t2 ON t1.id = t2.student_id";
    let mut stmt = conn.prepare(query_sql).expect("sql预处理出错");
    let student_group_iter = stmt
        .query_map([], |row| {
            Ok(StudentWithGroupIdVO {
                group_id: row.get(0)?,
                student_id: row.get(1)?,
                student_name: row.get(2)?,
                student_no: row.get(3)?,
            })
        })
        .expect("转换结果出错")
        .map(|item| item.unwrap());

    Ok(student_group_iter.collect())
}
