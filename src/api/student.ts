import { invoke } from "@tauri-apps/api";

export type Student = {
  id: string;
  stu_no: string;
  name: string;
};

export type StudentQueryVO = Partial<Pick<Student, "name" | "stu_no">>;

export type StudentCreateVO = Pick<Student, "stu_no" | "name">;

export type StudentUpdateVO = Pick<Student, "id"> &
  Partial<Omit<Student, "id" | "is_delete">>;

/**
 * 根据查询参数获取学生列表
 *
 * @param query - 查询参数
 * @return 一个包含一个Student数组的Promise
 */
export const getStudentList = (query: StudentQueryVO) => {
  const studentQueryVo: any = {};
  Object.entries(query).map(
    ([key, value]) => value && (studentQueryVo[key] = value)
  );

  return invoke<Student[]>("get_student_list", { studentQueryVo });
};

/**
 * 根据学生的ID获取学生
 *
 * @param id 学生的ID
 * @return 学生对象
 */
export const getStudentById = (id: Student["id"]) =>
  invoke<Student>("get_student_by_id", { id });

/**
 * 创建一个新的学生
 *
 * @param studentCreateVo - 学生信息
 * @return 一个包含一个学生ID的Promise
 */
export const createStudent = (studentCreateVo: StudentCreateVO) =>
  invoke<Student["id"]>("create_student", { studentCreateVo });

/**
 * 批量创建学生
 *
 * @param studentCreateVos - 一个包含多个学生信息对象的数组
 * @return 一个包含多个学生ID的Promise
 */
export const batchCreateStudent = (studentCreateVos: StudentCreateVO[]) =>
  invoke<Student["id"][]>("batch_create_student", { studentCreateVos });

/**
 * 更新学生
 *
 * @param studentUpdateVo - 需要更新的学生信息
 * @return 如果更新成功返回true,否则返回false
 */
export const updateStudent = (studentUpdateVo: StudentUpdateVO) =>
  invoke<number>("update_student", { studentUpdateVo });

/**
 * 根据学生的ID删除学生。
 *
 * @param id - 待删除的学生的ID
 * @return 如果删除成功返回true,否则返回false
 */
export const deleteStudent = (id: Student["id"]) =>
  invoke<boolean>("delete_student", { id });
