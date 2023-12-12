<script lang="ts" setup>
import { useDevicesList, useUserMedia } from "@vueuse/core";
import Viewer from "./components/pixi-viewer.vue";

// 获取设备
const { audioInputs: microphones } = useDevicesList({
  requestPermissions: true,
  constraints: { audio: true },
});
const currentMicrophone = computed(() => microphones.value[0]?.deviceId);

// 获取流
const { stream } = useUserMedia({
  constraints: {
    // @ts-ignore
    audio: { deviceId: currentMicrophone },
  },
  enabled: true,
});
</script>

<template>
  <n-el class="w-full h-full flex flex-col">
    <viewer v-if="stream" :stream="stream" />
    <n-empty v-if="!microphones.length" description="没有麦克风" />
  </n-el>
</template>
