<script setup lang="ts">
import TitleBar from "@/components/title-bar.vue";
</script>

<template>
  <div class="h-screen flex flex-col c-gray-7">
    <title-bar v-if="false" class="shrink-0" />
    <n-message-provider>
      <div class="grow basis-0 bg-white of-auto">
        <router-view v-slot="{ Component }">
          <!-- 缓存路由 -->
          <keep-alive>
            <component v-if="$route.meta.keepAlive" :is="Component" />
          </keep-alive>

          <!-- 普通路由 -->
          <component v-if="!$route.meta.keepAlive" :is="Component" />
        </router-view>
      </div>
    </n-message-provider>
  </div>
</template>
