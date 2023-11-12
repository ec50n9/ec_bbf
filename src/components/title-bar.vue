<script lang="ts" setup>
import { appWindow } from "@tauri-apps/api/window";
import {
  MinimizeRound,
  MaximizeRound,
  CloseRound,
  // ArrowBackRound,
} from "@vicons/material";
// import { useRouter } from "vue-router";

defineProps<{
  title: string;
}>();

// const router = useRouter();
</script>

<template>
  <div data-tauri-drag-region class="px-5 py-2 flex items-center gap-3 b-b">
    <!-- 返回按钮 -->
    <!-- <NButton class="shrink-0" quaternary circle @click="router.back()">
      <template #icon>
        <NIcon class="c-gray-7"><ArrowBackRound /></NIcon>
      </template>
    </NButton> -->

    <div class="text-2xl i-fluent-emoji:avocado" />

    <!-- 标题 -->
    <h1 data-tauri-drag-region class="grow basis-0 text-xl">{{ title }}</h1>

    <!-- 操作 -->
    <NSpace class="shrink-0">
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
