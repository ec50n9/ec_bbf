<script lang="ts" setup>
import {
  PauseRound as PauseIcon,
  PlayArrowRound as PlayIcon,
  RefreshRound as ResetIcon,
  CheckCircleOutlineRound as CheckIcon,
} from "@vicons/material";
import { PickStatus } from "@/composables/pick";
import { useAppStore } from "@/store/modules/app";

const appStore = useAppStore();

const props = defineProps<{
  pickStatus: PickStatus;
  content?: string;
}>();

const emit = defineEmits<{
  (e: "run"): void;
  (e: "select"): void;
  (e: "reset"): void;
  (e: "pause"): void;
}>();

const autoReset = ref(false);
const autoPause = ref(true);
watch(autoReset, (value) => {
  if (value) autoPause.value = true;
});
watch(autoPause, (value) => {
  if (!value) autoReset.value = false;
});

const runBtnText = computed(() => {
  if (props.pickStatus === "idle" || autoReset.value) return "开始";
  else if (props.pickStatus === "paused") return "继续";
});

const handleRun = () => {
  autoReset.value && emit("reset");
  emit("run");
};

const handleSelect = () => {
  emit("select");
  autoPause.value && emit("pause");
};

const handleReset = () => emit("reset");

const handlePause = () => emit("pause");
</script>

<template>
  <n-el class="px-1 pb-1 flex items-center justify-between gap-3">
    <n-el>
      <n-space align="center">
        <n-button
          v-if="['idle', 'paused'].includes(pickStatus)"
          type="primary"
          :size="appStore.miniWindowMode ? 'small' : 'medium'"
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
          v-if="['running'].includes(pickStatus)"
          type="info"
          :size="appStore.miniWindowMode ? 'small' : 'medium'"
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
          v-if="['running'].includes(pickStatus)"
          type="warning"
          :size="appStore.miniWindowMode ? 'small' : 'medium'"
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
          v-if="pickStatus !== 'idle'"
          type="error"
          :size="appStore.miniWindowMode ? 'small' : 'medium'"
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
    </n-el>

    <n-el v-if="content" class="shrink-0 text-3xl c-gray-7">{{ content }}</n-el>
  </n-el>
</template>
