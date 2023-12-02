import { invoke } from "@tauri-apps/api";
import { Student } from "./student";

export type StudentGroup = {
  id?: string;
  name: string;
  desc: string;
};

export const getStudentGroupList = () =>
  invoke<StudentGroup[]>("get_student_group_list");

export const getStudentGroupById = (id: StudentGroup["id"]) =>
  invoke<StudentGroup>("get_student_group_by_id", { id });

export type StudentGroupCreateVO = Pick<StudentGroup, "name" | "desc">;

export const createStudentGroup = (
  studentGroupCreateVo: StudentGroupCreateVO
) =>
  invoke<StudentGroup["id"]>("create_student_group", { studentGroupCreateVo });

export type StudentGroupUpdateVO = { id: string } & Partial<
  Pick<StudentGroup, "name" | "desc">
>;

export const updateStudentGroup = (
  studentGroupUpdateVo: StudentGroupUpdateVO
) => invoke<number>("update_student_group", { studentGroupUpdateVo });

export const deleteStudentGroup = (id: StudentGroup["id"]) =>
  invoke<number>("delete_student_group", { id });

export const bindStudentGroup = (
  studentId: Student["id"],
  groupId: StudentGroup["id"]
) => invoke<number>("bind_student_group", { studentId, groupId });

export const unbindStudentGroup = (
  studentId: Student["id"],
  groupId: StudentGroup["id"]
) => invoke<number>("unbind_student_group", { studentId, groupId });

export type StudentWithGroupIdVO = {
  group_id: StudentGroup["id"] | null;
  student_id: Student["id"];
  student_name: Student["name"];
  student_no: Student["stu_no"];
};

export const getAllStudentGroupMapping = () =>
  invoke<StudentWithGroupIdVO[]>("get_all_student_group_mapping");

export type StudentGroupBindVO = { student_id: string; group_id: string };
export type StudentGroupUnbindVO = { student_id: string };
export type StudentGroupUpdateRelVO = {
  student_id: string;
  update_to_group_id: string;
};

export const batchUpdateStudentGroupRel = (data: {
  needBindList: StudentGroupBindVO[];
  needUnbindList: StudentGroupUnbindVO[];
  needUpdateRelList: StudentGroupUpdateRelVO[];
}) => invoke<number>("batch_update_student_group_rel", data);
