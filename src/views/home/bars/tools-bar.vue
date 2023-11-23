<script lang="ts" setup>
import { FormatListBulletedRound as ScoreTypeIcon, AvTimerRound as TimerIcon } from "@vicons/material";
import { WebviewWindow } from "@tauri-apps/api/window";

const openScoreTypeManagerWindow = () => {
  const webview = new WebviewWindow("score-type-manager", {
    title: "分数类型管理",
    url: "/score-type-manager",
  });
  webview.once("tauri://created", () => {
    console.log("score type manager window created");
  });
  webview.once("tauri://error", (e) => {
    console.log("score type manager window error: ", e);
  });
};

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
      round
      @click="openScoreTypeManagerWindow"
    >
      <template #icon>
        <n-icon><score-type-icon /></n-icon>
      </template>
      分数类型管理
    </n-button>

    <n-button type="info" secondary round @click="openTimerWindow">
      <template #icon>
        <n-icon><timer-icon /></n-icon>
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
  </n-space>
</template>
