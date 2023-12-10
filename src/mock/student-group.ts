import { defineMock } from "@alova/mock";
import { invoke } from "@tauri-apps/api";

export default defineMock({
  "/student-groups": () => invoke("get_student_group_list"),
  "/student-groups/{id}": ({ params }) =>
    invoke("get_student_group_by_id", params),
  "[POST]/student-groups": ({ data: studentGroupCreateVo }) =>
    invoke("create_student_group", { studentGroupCreateVo }),
  "[PUT]/student-groups": ({ data: studentGroupUpdateVo }) =>
    invoke("update_student_group", { studentGroupUpdateVo }),
  "[DELETE]/student-groups/{id}": ({ params }) =>
    invoke("delete_student_group", params),

  "[POST]/bind-student-group": ({ data: studentGroupBindVo }) =>
    invoke("bind_student_group", { studentGroupBindVo }),
  "[POST]/unbind-student-group": ({ data: studentGroupUnbindVo }) =>
    invoke("unbind_student_group", { studentGroupUnbindVo }),
  "/student-group-mapping": () => invoke("get_all_student_group_mapping"),
  "[PUT]/student-group-mapping": ({
    data: { needBindList, needUnbindList, needUpdateRelList },
  }) =>
    invoke("batch_update_student_group_rel", {
      needBindList,
      needUnbindList,
      needUpdateRelList,
    }),
});
