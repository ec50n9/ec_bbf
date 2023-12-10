import { invoke } from "@tauri-apps/api";
import { defineMock } from "@alova/mock";

export default defineMock({
  "/score-types": () => invoke("get_score_type_list"),
  "/score-types/{id}": ({ params }) => invoke("get_score_type_by_id", params),
  "[POST]/score-types": ({ data: scoreTypeCreateVo }) =>
    invoke("create_score_type", { scoreTypeCreateVo }),
  "[PUT]/score-types": ({ data: scoreTypeUpdateVo }) =>
    invoke("update_score_type", { scoreTypeUpdateVo }),
  "[DELETE]/score-types/{id}": ({ params }) =>
    invoke("delete_score_type", params),

  "[POST]/add-score": ({ data: scorePlusVo }) =>
    invoke("add_score", { scorePlusVo }),
  "/score-list/{studentId}": ({ params }) =>
    invoke("get_score_list_by_student_id", params),
  "/daily-score/{studentId}": ({ params, query }) =>
    invoke("get_daily_score_by_student_id", { ...params, ...query }),
});
