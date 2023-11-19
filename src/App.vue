<script setup lang="ts">
import TitleBar from "@/components/title-bar.vue";
</script>

<template>
  <div class="h-screen flex flex-col c-gray-7">
    <title-bar class="shrink-0" />
    <n-message-provider>
      <div class="grow basis-0 bg-white of-auto">
        <router-view />
      </div>
    </n-message-provider>
  </div>
</template>
