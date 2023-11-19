<script lang="ts" setup>
import { FormatListBulletedRound as ScoreTypeIcon } from "@vicons/material";
</script>

<template>
  <n-space class="px-1 pb-1">
    <n-button type="primary" secondary round>
      <template #icon><score-type-icon /></template>
      配置分数类型
    </n-button>
  </n-space>
</template>
