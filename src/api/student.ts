import { invoke } from "@tauri-apps/api";

export type Student = {
  id: string;
  stu_no: string;
  name: string;
  is_delete: boolean;
};

export type StudentCreateVO = Pick<Student, "id" | "stu_no" | "name">;

export type StudentUpdateVO = Pick<Student, "id"> &
  Partial<Omit<Student, "id" | "is_delete">>;

export const getStudentList = () => invoke<Student[]>("get_student_list");

export const createStudent = (studentCreateVo: StudentCreateVO) =>
  invoke<Student["id"]>("create_student", { studentCreateVo });

export const updateStudent = (studentUpdateVo: StudentUpdateVO) =>
  invoke<boolean>("update_student", { studentUpdateVo });

export const deleteStudent = (id: Student["id"]) =>
  invoke<boolean>("delete_student", { id });
