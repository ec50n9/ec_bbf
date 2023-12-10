export type Student = {
  id: string;
  stu_no: string;
  name: string;
};

export type StudentQueryVO = Partial<Pick<Student, "name" | "stu_no">>;

export type StudentCreateVO = Pick<Student, "stu_no" | "name">;

export type StudentUpdateVO = Pick<Student, "id"> &
  Partial<Omit<Student, "id" | "is_delete">>;
