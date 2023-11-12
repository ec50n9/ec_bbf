import { invoke } from "@tauri-apps/api";

export type ScoreType = {
  id: string;
  name: string;
  desc: string;
  max: number;
};

/**
 * 获取全部分数类型
 *
 * @return 分数类型列表
 */
export const getScoreTypeList = () =>
  invoke<ScoreType[]>("get_score_type_list");

export type ScoreTypeCreateVO = {
  name: string;
  desc: string;
  max: number;
};

/**
 * 创建一个分数类型
 *
 * @param scoreTypeCreateVo - 分数类型信息
 * @return 一个包含一个分数类型ID的Promise
 */
export const createScoreType = (scoreTypeCreateVo: ScoreTypeCreateVO) =>
  invoke<ScoreType["id"]>("create_score_type", { scoreTypeCreateVo });

export type ScoreTypeUpdateVO = {
  id: string;
  name?: string;
  desc?: string;
  max?: number;
};

/**
 * 更新一个分数类型
 *
 * @param scoreTypeUpdateVo - 分数类型信息
 * @return 更新行数
 */
export const updateScoreType = (scoreTypeUpdateVo: ScoreTypeUpdateVO) =>
  invoke<number>("update_score_type", { scoreTypeUpdateVo });

/**
 * 删除一个分数类型
 *
 * @param id - 需要删除的分数类型ID
 * @return 删除行数
 */
export const deleteScoreType = (id: ScoreType["id"]) =>
  invoke<number>("delete_score_type", { id });

export type ScorePlusVO = {
  student_id: string;
  score_type_id: string;
  action_value: number;
  reason?: string;
};

/**
 * 为学生加分
 *
 * @param scorePlusVo - 分数信息
 * @return 修改行数
 */
export const addScore = (scorePlusVo: ScorePlusVO) =>
  invoke<number>("add_score", { scorePlusVo });

export type Score = {
  score_type_id: string;
  score: number;
};

/**
 * 根据学生的ID获取学生全部分数
 *
 * @param student_id - 学生的ID
 * @return 学生全部分数
 */
export const getScoreListByStudentId = (studentId: string) =>
  invoke<Score[]>("get_score_list_by_student_id", { studentId });

export type ScoreStatistics = {
  date: string;
  positive_count: number;
  negative_count: number;
  positive_total_score: number;
  negative_total_score: number;
};

export const getDailyScoreByStudentId = (
  studentId: string,
  startDate: string,
  endDate: string
) =>
  invoke<ScoreStatistics[]>("get_daily_score_by_student_id", {
    studentId,
    startDate,
    endDate,
  });
