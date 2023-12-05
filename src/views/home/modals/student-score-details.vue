<script lang="ts" setup>
import { Student } from "@/api/student";
import {
  Score,
  ScoreType,
  getScoreListByStudentId,
  getScoreTypeList,
  addScore,
} from "@/api/score";
import { getStudentById } from "@/api/student";
import {
  DeleteOutlineRound as DeleteIcon,
  ModeOutlined as EditIcon,
} from "@vicons/material";
import ScoreActionModal from "./score-action-modal.vue";

const emit = defineEmits<{
  (e: "delete", id: Student["id"]): void;
  (e: "edit", id: Student["id"]): void;
}>();

const studentId = ref<Student["id"]>();
const studentInfo = ref<Student>();
const scoreTypeList = ref<ScoreType[]>([]);
const scoreMap = ref<Map<Score["score_type_id"], Score["score"]>>(new Map());

// 刷新数据
const refreshData = async () => {
  scoreTypeList.value = await getScoreTypeList();
  if (!studentId.value) return;
  const scoreList = await getScoreListByStudentId(studentId.value);
  scoreMap.value = scoreList.reduce((map, curr) => {
    map.set(curr.score_type_id, curr.score);
    return map;
  }, new Map());
};

// 弹窗相关
const visible = ref(false);
const open = async (id: Student["id"]) => {
  visible.value = true;
  studentId.value = id;
  studentInfo.value = await getStudentById(id);
  await refreshData();

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
            currentScore: scoreMap.get(item.id!) || 0,
          })
        "
      >
        <n-el class="text-2xl">{{ scoreMap.get(item.id!) || 0 }}</n-el>
        <n-el>{{ item.name }}</n-el>
      </n-el>
    </n-el>
  </n-modal>

  <score-action-modal
    ref="scoreActionModalRef"
    @submit="
      (scoreTypeId, value, reason) => handlePlus(scoreTypeId, value, reason)
    "
  />
</template>
