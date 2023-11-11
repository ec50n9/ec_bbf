<script lang="ts" setup>
import { Student } from "@/api/student";
import { getScoreListByStudentId, getScoreTypeList } from "@/api/score";

const visible = ref(false);

const open = async (id: Student["id"]) => {
  visible.value = true;
  const scoreTypeList = await getScoreTypeList();
  const scoreList = await getScoreListByStudentId(id);
  console.log(scoreTypeList, scoreList);
};
defineExpose({ open });
</script>

<template>
  <n-modal
    v-model:show="visible"
    class="w-4/5 max-w-120"
    preset="card"
    title="你好"
  >
    这里是内容
  </n-modal>
</template>
