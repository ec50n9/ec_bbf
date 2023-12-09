<script lang="ts" setup>
import { Score, ScoreType } from "@/apis/types/score";
import { Student } from "@/apis/types/student";
import * as ScoreApi from "@/apis/modules/score";
import * as StudentApi from "@/apis/modules/student";
import {
  DeleteOutlineRound as DeleteIcon,
  ModeOutlined as EditIcon,
} from "@vicons/material";
import ScoreActionModal from "./score-action-modal.vue";
import { useRequest } from "alova";

const emit = defineEmits<{
  (e: "delete", id: Student["id"]): void;
  (e: "edit", id: Student["id"]): void;
}>();

const studentId = ref<Student["id"]>();
const studentInfo = ref<Student>();

// 分数类型列表
const {
  data: scoreTypeList,
  loading: scoreTypeListLoading,
  send: getScoreTypeList,
} = useRequest(ScoreApi.getScoreTypeList, {
  immediate: false,
});

// 分数列表
const {
  data: scoreList,
  loading: scoreListLoading,
  send: getScoreListByStudentId,
} = useRequest(ScoreApi.getScoreListByStudentId, {
  immediate: false,
});

// 分数类型和分数对应map
const scoreMap = computed<
  Map<Score["score_type_id"], Score["score"]> | undefined
>(() =>
  scoreList.value?.reduce((map, curr) => {
    map.set(curr.score_type_id, curr.score);
    return map;
  }, new Map())
);

// 刷新数据
const refreshData = async () => {
  getScoreTypeList();
  if (!studentId.value) return;
  getScoreListByStudentId(studentId.value);
};

// 弹窗相关
const { send: getStudentById } = useRequest(StudentApi.getStudentById, {
  immediate: false,
});
const visible = ref(false);
const open = async (id: Student["id"]) => {
  visible.value = true;
  studentId.value = id;
  refreshData();
  studentInfo.value = await getStudentById(id);

  // const test = await getDailyScoreByStudentId(id, "2023-09-01", "2024-09-01");
};
defineExpose({ open });

/** 分数编辑弹窗 */
const scoreActionModalRef = ref<InstanceType<typeof ScoreActionModal>>();

const handleDelete = (studentId: Student["id"]) => {
  emit("delete", studentId);
  visible.value = false;
};

/** 加分 */
const { send: addScore } = useRequest(ScoreApi.addScore, {
  immediate: false,
});
const handlePlus = async (
  scoreTypeId: ScoreType["id"],
  plusValue: number,
  reason?: string
) => {
  await addScore({
    student_id: studentId.value!,
    score_type_id: scoreTypeId!,
    action_value: plusValue,
    reason,
  });
  await refreshData();
};
</script>

<template>
  <n-modal
    v-model:show="visible"
    class="w-4/5 max-w-120"
    preset="card"
    :title="studentInfo?.name"
    size="medium"
    :bordered="false"
    :segmented="{
      content: false,
      footer: false,
    }"
  >
    <template #header-extra>
      <n-space class="mr-3">
        <!-- 编辑 -->
        <n-button
          secondary
          circle
          size="small"
          type="warning"
          @click.stop="emit('edit', studentId!)"
        >
          <template #icon>
            <n-icon><edit-icon /></n-icon>
          </template>
        </n-button>
        <!-- 删除 -->
        <n-popconfirm
          negative-text="取消"
          positive-text="确定"
          @positive-click="handleDelete(studentId!)"
        >
          <template #trigger>
            <n-button secondary circle size="small" type="error" @click.stop>
              <template #icon>
                <n-icon><delete-icon /></n-icon>
              </template>
            </n-button>
          </template>
          删除了就没有了喔
        </n-popconfirm>
      </n-space>
    </template>

    <!-- 主体 -->
    <n-spin :show="scoreTypeListLoading || scoreListLoading">
      <n-el class="grid grid-cols-3 gap-3">
        <n-el
          v-for="item in scoreTypeList"
          :key="item.id"
          class="p-3 flex flex-col items-center c-white rd-3"
          :style="{ color: item.color || '#000' }"
          @click="
            scoreActionModalRef?.open({
              action: 'plus',
              studentId: studentInfo!.id,
              studentName: studentInfo!.name,
              scoreTypeId: item.id,
              scoreTypeName: item.name,
              currentScore: scoreMap?.get(item.id!) || 0,
            })
          "
        >
          <n-el class="text-2xl">{{ scoreMap?.get(item.id!) || 0 }}</n-el>
          <n-el>{{ item.name }}</n-el>
        </n-el>
      </n-el>
    </n-spin>
  </n-modal>

  <score-action-modal
    ref="scoreActionModalRef"
    @submit="
      (scoreTypeId, value, reason) => handlePlus(scoreTypeId, value, reason)
    "
  />
</template>
