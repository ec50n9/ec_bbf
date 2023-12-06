import {
  Score,
  ScorePlusVO,
  ScoreStatistics,
  ScoreType,
  ScoreTypeCreateVO,
  ScoreTypeUpdateVO,
} from "@/apis/types/score";
import { request } from "..";

/**
 * 获取全部分数类型
 *
 * @return 分数类型列表
 */
export const getScoreTypeList = () =>
  request.Get<ScoreType[]>("/score-types", {
    name: "score-type-list",
    hitSource: /^score-type/,
  });

/**
 * 根据分数类型的ID获取分数类型
 *
 * @param id - 分数类型的ID
 * @return 分数类型
 */
export const getScoreTypeById = (id: ScoreType["id"]) =>
  request.Get<ScoreType>(`/score-types/${id}`, { name: "score-type-by-id" });

/**
 * 创建一个分数类型
 *
 * @param scoreTypeCreateVo - 分数类型信息
 * @return 一个包含一个分数类型ID的Promise
 */
export const createScoreType = (data: ScoreTypeCreateVO) =>
  request.Post<ScoreType["id"]>("/score-types", data, {
    name: "score-type-create",
  });

/**
 * 更新一个分数类型
 *
 * @param scoreTypeUpdateVo - 分数类型信息
 * @return 更新行数
 */
export const updateScoreType = (data: ScoreTypeUpdateVO) =>
  request.Put<number>("/score-types", data, {
    name: "score-type-update",
  });

/**
 * 删除一个分数类型
 *
 * @param id - 需要删除的分数类型ID
 * @return 删除行数
 */
export const deleteScoreType = (id: ScoreType["id"]) =>
  request.Delete<number>(
    `/score-types/${id}`,
    {},
    {
      name: "score-type-delete",
    }
  );

/**
 * 为学生加分
 *
 * @param scorePlusVo - 分数信息
 * @return 修改行数
 */
export const addScore = (data: ScorePlusVO) =>
  request.Post<number>("/add-score", data);

/**
 * 根据学生的ID获取学生全部分数
 *
 * @param student_id - 学生的ID
 * @return 学生全部分数
 */
export const getScoreListByStudentId = (studentId: string) =>
  request.Get<Score[]>(`/score-list/${studentId}`);

export const getDailyScoreByStudentId = (
  studentId: string,
  startDate: string,
  endDate: string
) =>
  request.Get<ScoreStatistics[]>(`/daily-score/${studentId}`, {
    params: { startDate, endDate },
  });
