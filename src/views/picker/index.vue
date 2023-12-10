<script lang="ts" setup>
import {
  PauseRound as PauseIcon,
  PlayArrowRound as PlayIcon,
  RefreshRound as ResetIcon,
  CheckCircleOutlineRound as CheckIcon,
} from "@vicons/material";
import { useRequest } from "alova";
import * as StudentApi from "@/api/modules/student";
import { usePick } from "@/composables/pick";

const { data: studentList } = useRequest(() => StudentApi.getStudentList({}), {
  initialData: [],
});

const studentIds = computed(() =>
  studentList.value.map((student) => student.id)
);
const studentIdMap = computed(
  () => new Map(studentList.value.map((student) => [student.id, student]))
);

const picker = usePick(studentIds);

const autoReset = ref(false);
const autoPause = ref(true);
watch(autoReset, (value) => {
  if (value) autoPause.value = true;
});
watch(autoPause, (value) => {
  if (!value) autoReset.value = false;
});

const runBtnText = computed(() => {
  if (picker.status.value === "idle" || autoReset.value) return "开始";
  else if (picker.status.value === "paused") return "继续";
});

const handleRun = () => {
  autoReset.value && picker.reset();
  picker.run();
};

const handleSelect = () => {
  picker.select();
  autoPause.value && picker.pause();
};

const handleReset = () => picker.reset();

const handlePause = () => picker.pause();
</script>

<template>
  <n-el class="px-1 pb-1">
    <n-space align="center">
      <n-button
        v-if="['idle', 'paused'].includes(picker.status.value)"
        type="primary"
        secondary
        round
        @click="handleRun"
      >
        <template #icon>
          <n-icon><play-icon /></n-icon>
        </template>
        {{ runBtnText }}
      </n-button>
      <n-button
        v-if="['running'].includes(picker.status.value)"
        type="info"
        secondary
        round
        @click="handleSelect"
      >
        <template #icon>
          <n-icon><check-icon /></n-icon>
        </template>
        抽取
      </n-button>
      <n-button
        v-if="['running'].includes(picker.status.value)"
        type="warning"
        secondary
        round
        @click="handlePause"
      >
        <template #icon>
          <n-icon><pause-icon /></n-icon>
        </template>
        暂停
      </n-button>
      <n-button
        v-if="picker.status.value !== 'idle'"
        type="error"
        secondary
        round
        @click="handleReset"
      >
        <template #icon>
          <n-icon><reset-icon /></n-icon>
        </template>
        重置
      </n-button>
    </n-space>
    <n-space class="mt-3">
      <n-checkbox v-model:checked="autoReset">可重复抽取</n-checkbox>
      <n-checkbox v-model:checked="autoPause">自动暂停</n-checkbox>
    </n-space>
    <n-space v-if="picker.selectedList.value.size" align="center" class="mt-2">
      <n-el>已选择的学生:</n-el>
      <n-tag
        v-for="studentId in picker.selectedList.value"
        :key="studentId"
        type="info"
        @click="openStudentScoreDetailsModal(studentId)"
        closable
        @close="picker.unselect(studentId)"
      >
        {{ studentIdMap.get(studentId)?.name }}
      </n-tag>
      <n-tag type="error" @click="picker.reset()">
        <template #icon>
          <n-icon><reset-icon /></n-icon>
        </template>
        重置
      </n-tag>
    </n-space>
  </n-el>
</template>
