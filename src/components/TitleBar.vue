<script lang="ts" setup>
import { appWindow } from "@tauri-apps/api/window";
import { NIcon, NButton, NSpace } from "naive-ui";
import { MinimizeRound, MaximizeRound, CloseRound } from "@vicons/material";

defineProps<{
  title: string;
}>();
</script>

<template>
  <div
    data-tauri-drag-region
    class="px-5 py-3 flex justify-between items-center gap-3 b-b"
  >
    <!-- 标题 -->
    <h1 class="text-xl">{{ title }}</h1>

    <!-- 操作 -->
    <NSpace>
      <!-- 最小化 -->
      <NButton
        strong
        secondary
        circle
        type="info"
        size="small"
        @click="appWindow.minimize()"
      >
        <template #icon>
          <NIcon><MinimizeRound /></NIcon>
        </template>
      </NButton>

      <!-- 最大化 -->
      <NButton
        strong
        secondary
        circle
        type="primary"
        size="small"
        @click="appWindow.toggleMaximize()"
      >
        <template #icon>
          <NIcon><MaximizeRound /></NIcon>
        </template>
      </NButton>

      <!-- 关闭 -->
      <NButton
        strong
        secondary
        circle
        type="error"
        size="small"
        @click="appWindow.close()"
      >
        <template #icon>
          <NIcon><CloseRound /></NIcon>
        </template>
      </NButton>
    </NSpace>
  </div>
</template>
