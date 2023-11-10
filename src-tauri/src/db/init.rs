use rusqlite::{Connection, Result};

/// 初始化数据库
pub fn init_db() -> Result<Connection> {
    let db_path = "D://db.sqlite";
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS student (
          id          VARCHAR(64)     PRIMARY KEY,
          stu_no      VARCHAR(20)     NOT NULL,
          name        VARCHAR(20)     NOT NULL
      );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS score_type (
          id          VARCHAR(64)     PRIMARY KEY,
          name        VARCHAR(20)     NOT NULL,
          desc        VARCHAR(100)    DEFAULT '',
          max         INTEGER         NOT NULL DEFAULT -1
      );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS student_score_mapping (
          student_id      VARCHAR(64)     NOT NULL,
          score_type_id   VARCHAR(64)     NOT NULL,
          score           INTEGER         NOT NULL DEFAULT 0 
      );",
        [],
    )?;
    Ok(conn)
}
