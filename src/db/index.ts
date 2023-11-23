import Database from "tauri-plugin-sql-api";
import initSql from "@/assets/sql/init.sql?raw";

let db: Database | null = null;

const initDB = async (db: Database) => {
  console.log("init db...");
  const result = await db.execute(initSql);
  console.log(result);
  console.log("init db success!");
};

export const useDB = async () => {
  if (!db) {
    db = await Database.load("sqlite:ec-bbf.db");
    await initDB(db);
  }
  return db;
};
