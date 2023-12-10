import {
  Student,
  StudentCreateVO,
  StudentQueryVO,
  StudentUpdateVO,
} from "@/api/types/student";
import { request } from "..";

/**
 * 根据查询参数获取学生列表
 *
 * @param query - 查询参数
 * @return 一个包含一个Student数组的Promise
 */
export const getStudentList = (query: StudentQueryVO) =>
  request.Get<Student[]>("students", {
    params: Object.entries(query).reduce((acc, [key, value]) => {
      if (value) acc[key] = value;
      return acc;
    }, {} as Record<string, any>),
    name: "student-list",
    hitSource: /^student/,
  });

/**
 * 根据学生的ID获取学生
 *
 * @param id 学生的ID
 * @return 学生对象
 */
export const getStudentById = (id: Student["id"]) =>
  request.Get<Student>(`students/${id}`, { name: "student-by-id" });

/**
 * 创建一个新的学生
 *
 * @param studentCreateVo - 学生信息
 * @return 一个包含一个学生ID的Promise
 */
export const createStudent = (studentCreateVo: StudentCreateVO) =>
  request.Post<Student["id"]>("students", studentCreateVo, {
    name: "student-create",
  });

/**
 * 批量创建学生
 *
 * @param studentCreateVos - 一个包含多个学生信息对象的数组
 * @return 一个包含多个学生ID的Promise
 */
export const batchCreateStudent = (studentCreateVos: StudentCreateVO[]) =>
  request.Post<Student["id"][]>("students/batch", studentCreateVos, {
    name: "student-batch-create",
  });

/**
 * 更新学生
 *
 * @param studentUpdateVo - 需要更新的学生信息
 * @return 如果更新成功返回true,否则返回false
 */
export const updateStudent = (studentUpdateVo: StudentUpdateVO) =>
  request.Put<boolean>("students", studentUpdateVo, { name: "student-update" });

/**
 * 根据学生的ID删除学生。
 *
 * @param id - 待删除的学生的ID
 * @return 如果删除成功返回true,否则返回false
 */
export const deleteStudent = (id: Student["id"]) =>
  request.Delete<boolean>(`students/${id}`, {}, { name: "student-delete" });
