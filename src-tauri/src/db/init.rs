use rusqlite::{Connection, Result};

/// 初始化数据库
pub fn init_db() -> Result<Connection> {
    let db_path = "D://db.sqlite";
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS student (
                  id          TEXT     PRIMARY KEY,
                  stu_no      TEXT     NOT NULL,
                  name        TEXT     NOT NULL
              );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS score_type (
                  id          TEXT     PRIMARY KEY,
                  name        TEXT     NOT NULL,
                  desc        TEXT    DEFAULT '',
                  max         INTEGER         NOT NULL DEFAULT -1
              );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS student_score_mapping (
                  student_id      TEXT     NOT NULL,
                  score_type_id   TEXT     NOT NULL,
                  score           INTEGER  NOT NULL DEFAULT 0 
              );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS student_score_record (
                  id            TEXT    PRIMARY KEY,
                  student_id    TEXT    NOT NULL,
                  score_type_id TEXT    NOT NULL,
                  action_value  INTEGER NOT NULL,
                  reason        TEXT    NOT NULL,
                  create_time   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
              );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS student_group (
            id          TEXT     PRIMARY KEY,
            name        TEXT     NOT NULL,
            desc        TEXT     NOT NULL DEFAULT ''
        );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS student_group_mapping (
            student_id  TEXT     NOT NULL,
            group_id    TEXT     NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}
