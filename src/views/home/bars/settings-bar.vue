<script lang="ts" setup>
import { FormatListBulletedRound as ScoreTypeIcon } from "@vicons/material";
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

    <n-button
      type="info"
      secondary
      @click="$router.push('/group-manager')"
    >
      <template #icon>
        <n-icon><score-type-icon /></n-icon>
      </template>
      分组管理
    </n-button>

    <n-button
      type="warning"
      secondary
      @click="$router.push('/student-manager')"
    >
      <template #icon>
        <n-icon><score-type-icon /></n-icon>
      </template>
      学生管理
    </n-button>
  </n-space>
</template>
