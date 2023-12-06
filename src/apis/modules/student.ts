import { Student, StudentQueryVO } from "@/apis/types/student";
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
  });
