import { invoke } from "@tauri-apps/api";

export type Student = {
  id: string;
  stu_no: string;
  name: string;
  is_delete: boolean;
};

export type StudentQueryVO = Partial<Pick<Student, "name" | "stu_no">>;

export type StudentCreateVO = Pick<Student, "stu_no" | "name">;

export type StudentUpdateVO = Pick<Student, "id"> &
  Partial<Omit<Student, "id" | "is_delete">>;

export const getStudentList = (studentQueryVo: StudentQueryVO) =>
  invoke<Student[]>("get_student_list", { studentQueryVo });

export const createStudent = (studentCreateVo: StudentCreateVO) =>
  invoke<Student["id"]>("create_student", { studentCreateVo });

export const updateStudent = (studentUpdateVo: StudentUpdateVO) =>
  invoke<boolean>("update_student", { studentUpdateVo });

export const deleteStudent = (id: Student["id"]) =>
  invoke<boolean>("delete_student", { id });
