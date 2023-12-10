import { Student } from "./student";

export type StudentGroup = {
  id?: string;
  name: string;
  desc: string;
};

export type StudentGroupCreateVO = Pick<StudentGroup, "name" | "desc">;

export type StudentGroupUpdateVO = { id: string } & Partial<
  Pick<StudentGroup, "name" | "desc">
>;

export type StudentWithGroupIdVO = {
  group_id: StudentGroup["id"] | null;
  student_id: Student["id"];
  student_name: Student["name"];
  student_no: Student["stu_no"];
};

export type StudentGroupBindVO = { student_id: string; group_id: string };
export type StudentGroupUnbindVO = { student_id: string };
export type StudentGroupUpdateRelVO = {
  student_id: string;
  update_to_group_id: string;
};
