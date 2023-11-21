<script lang="ts" setup>
import { FormatListBulletedRound as ScoreTypeIcon } from "@vicons/material";
import { WebviewWindow } from "@tauri-apps/api/window";

const openTimerWindow = () => {
  const webview = new WebviewWindow("timer", {
    title: "计时器",
    url: "/timer",
  });
  webview.once("tauri://created", () => {
    console.log("timer window created");
  });
  webview.once("tauri://error", (e) => {
    console.log("timer window error: ", e);
  });
};
</script>

<template>
  <n-space class="px-1 pb-1">
    <n-button
      type="primary"
      secondary
      @click="$router.push('/score-type-manager')"
    >
      <template #icon>
        <n-icon><score-type-icon /></n-icon>
      </template>
      分数类型管理
    </n-button>

    <n-button type="info" secondary @click="openTimerWindow">
      <template #icon>
        <n-icon><score-type-icon /></n-icon>
      </template>
      计时器
    </n-button>

    <!-- <n-button
      type="info"
      secondary
      @click="$router.push('/group-manager')"
    >
      <template #icon>
        <n-icon><score-type-icon /></n-icon>
      </template>
      分组管理
    </n-button> -->

    <!-- <n-button
      type="warning"
      secondary
      @click="$router.push('/student-manager')"
    >
      <template #icon>
        <n-icon><score-type-icon /></n-icon>
      </template>
      学生管理
    </n-button> -->
  </n-space>
</template>
