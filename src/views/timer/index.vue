<script lang="ts" setup>
import dayjs from "dayjs";
import utc from "dayjs/plugin/utc";
import {
  PauseRound as PauseIcon,
  PlayArrowRound as PlayIcon,
  RefreshRound as ResetIcon,
} from "@vicons/material";
import "animate.css";

/** 当前值 */
const current = ref(0);
/** 计时时长 */
const durationMin = ref(1);
/** 开始时间 */
const startTime = ref<number | null>(null);
/** 正计时还是倒计时 */
const countDown = ref(false);
/** 显示毫秒 */
const showMillisecond = ref(true);

const isEnd = computed(
  () => current.value === (countDown.value ? 0 : durationMin.value * 60 * 1000)
);

const running = ref(false);

const timeTemplate = computed(
  () => `HH:mm:ss${showMillisecond.value ? ".SSS" : ""}`
);

dayjs.extend(utc);
const timeStr = computed(() => {
  return dayjs(current.value).utc().format(timeTemplate.value);
});

const update = () => {
  if (!running.value) return requestAnimationFrame(update);

  const millisecond = Math.floor(Date.now() - startTime.value!);
  const durationMillisecond = durationMin.value * 60 * 1000;

  if (countDown.value) current.value = durationMillisecond - millisecond;
  else current.value = millisecond;

  if (
    (countDown.value && current.value > 0) ||
    (!countDown.value && current.value < durationMillisecond)
  )
    requestAnimationFrame(update);
  else {
    current.value = countDown.value ? 0 : durationMillisecond;
    // handleReset();
    handlePause();
    startTime.value = null;
  }
};

const handlePause = () => {
  running.value = false;
};

const handleStart = () => {
  handleReset();
  handleRun();
};

const handleRun = () => {
  running.value = true;
  if (!startTime.value) startTime.value = Date.now();
  else {
    startTime.value = Date.now() - current.value;
  }
  requestAnimationFrame(update);
};

const handleReset = () => {
  current.value = countDown.value ? durationMin.value * 60 * 1000 : 0;
  startTime.value = null;
  running.value = false;
};

watch(
  () => [durationMin.value, countDown.value],
  () => {
    handleReset();
  }
);
</script>

<template>
  <n-el
    class="p-3 h-full flex flex-col gap-3"
    :class="{ 'bg-orange-50': isEnd }"
  >
    <n-space
      size="large"
      class="shrink-0 px-5 py-3 b rd-3"
      :class="{ 'b-orange-5': isEnd }"
      align="center"
    >
      <n-space size="small" align="center">
        <n-el>时长</n-el>
        <n-input-number class="w-35" v-model:value="durationMin">
          <template #suffix>分钟</template>
        </n-input-number>
      </n-space>

      <n-space size="small" align="center">
        <n-el>计时方式</n-el>
        <n-radio-group v-model:value="countDown">
          <n-radio-button :value="false" label="正计时" />
          <n-radio-button :value="true" label="倒计时" />
        </n-radio-group>
      </n-space>

      <n-space size="small" align="center">
        <n-el>显示毫秒</n-el>
        <n-switch v-model:value="showMillisecond" />
      </n-space>
    </n-space>

    <n-el class="basis-0 grow flex flex-col items-center justify-center gap-3">
      <n-el
        class="text-5xl"
        :class="{
          'c-orange-7': isEnd,
          'animate__animated animate__bounce animate__infinite': isEnd,
        }"
        >{{ timeStr }}</n-el
      >
      <n-space>
        <n-button
          v-if="!startTime"
          type="primary"
          secondary
          round
          @click="handleStart"
        >
          <template #icon>
            <n-icon><play-icon /></n-icon>
          </template>
          开始
        </n-button>

        <template v-else>
          <n-button
            v-if="!running"
            type="info"
            secondary
            round
            @click="handleRun"
          >
            <template #icon>
              <n-icon><play-icon /></n-icon>
            </template>
            继续
          </n-button>

          <n-button v-else type="warning" secondary round @click="handlePause">
            <template #icon>
              <n-icon><pause-icon /></n-icon>
            </template>
            暂停
          </n-button>
        </template>

        <n-button type="error" secondary round @click="handleReset">
          <template #icon>
            <n-icon><reset-icon /></n-icon>
          </template>
          重置
        </n-button>
      </n-space>
    </n-el>
  </n-el>
</template>

<style scoped></style>
