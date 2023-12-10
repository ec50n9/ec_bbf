import { request } from "..";
import { Student } from "../types/student";
import {
  StudentGroup,
  StudentGroupBindVO,
  StudentGroupCreateVO,
  StudentGroupUnbindVO,
  StudentGroupUpdateRelVO,
  StudentGroupUpdateVO,
  StudentWithGroupIdVO,
} from "../types/student-group";

export const getStudentGroupList = () =>
  request.Get<StudentGroup[]>("/student-groups", {
    name: "student-group-list",
    hitSource: /^student-group/,
  });

export const getStudentGroupById = (id: StudentGroup["id"]) =>
  request.Get<StudentGroup>(`/student-groups/${id}`, {
    name: "student-group-by-id",
  });

export const createStudentGroup = (
  studentGroupCreateVo: StudentGroupCreateVO
) =>
  request.Post<StudentGroup["id"]>("/student-groups", studentGroupCreateVo, {
    name: "student-group-create",
  });

export const updateStudentGroup = (
  studentGroupUpdateVo: StudentGroupUpdateVO
) =>
  request.Put<number>("/student-groups", studentGroupUpdateVo, {
    name: "student-group-update",
  });

export const deleteStudentGroup = (id: StudentGroup["id"]) =>
  request.Delete<number>(`/student-groups/${id}`, {
    name: "student-group-delete",
  });

export const bindStudentGroup = (
  studentId: Student["id"],
  groupId: StudentGroup["id"]
) =>
  request.Post<number>(
    "/bind-student-group",
    { studentId, groupId },
    {
      name: "student-group-bind",
    }
  );

export const unbindStudentGroup = (
  studentId: Student["id"],
  groupId: StudentGroup["id"]
) =>
  request.Post<number>(
    "/unbind-student-group",
    {
      data: { studentId, groupId },
    },
    {
      name: "student-group-unbind",
    }
  );

export const getAllStudentGroupMapping = () =>
  request.Get<StudentWithGroupIdVO[]>("/student-group-mapping", {
    name: "student-group-mapping",
    hitSource: /^student-group-mapping/,
  });

export const batchUpdateStudentGroupRel = (data: {
  needBindList: StudentGroupBindVO[];
  needUnbindList: StudentGroupUnbindVO[];
  needUpdateRelList: StudentGroupUpdateRelVO[];
}) =>
  request.Put<number>("/student-group-mapping", data, {
    name: "student-group-mapping-update",
  });
