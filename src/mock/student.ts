import { invoke } from "@tauri-apps/api";
import { defineMock } from "@alova/mock";

export default defineMock({
  "/students": ({ query }) =>
    invoke("get_student_list", { studentQueryVo: query }),
  "/students/{id}": ({ params }) => invoke("get_student_by_id", params),
  "[POST]/students": ({ data: studentCreateVo }) =>
    invoke("create_student", { studentCreateVo }),
  "[POST]/students/batch": ({ data: studentCreateVos }) =>
    invoke("batch_create_student", { studentCreateVos }),
  "[PUT]/students": ({ data: studentUpdateVo }) =>
    invoke("update_student", { studentUpdateVo }),
  "[DELETE]/students/{id}": ({ params }) => invoke("delete_student", params),
});
