<script lang="ts" setup>
import { VueDraggable } from "vue-draggable-plus";

const groupList = ref([
  {
    name: "小组1",
    id: 1,
    students: [
      {
        name: "Joao",
        id: 1,
      },
      {
        name: "Jean",
        id: 2,
      },
    ],
  },
  {
    name: "小组2",
    id: 2,
    students: [],
  },
  {
    name: "小组3",
    id: 3,
    students: [],
  },
  {
    name: "小组4",
    id: 4,
    students: [],
  },
]);

/** 未分组 */
const ungroupStudentList = ref([
  {
    name: "Johanna",
    id: 3,
  },
  {
    name: "Juan",
    id: 4,
  },
  {
    name: "Jorge",
    id: 5,
  },
]);

const handleSave = () => {
  console.log("分组：", unref(groupList));
  console.log("未分组：", unref(ungroupStudentList));
};
</script>

<template>
  <n-layout has-sider sider-placement="right" class="h-full">
    <n-layout content-style="display: flex; flex-direction: column">
      <!-- 标题栏 -->
      <n-layout-header bordered>
        <n-space class="p-3">
          <n-button @click="handleSave">保存</n-button>
        </n-space>
      </n-layout-header>

      <!-- 分组列表 -->
      <n-layout-content content-style="padding: 24px;">
        <n-el class="flex flex-wrap justify-between gap-3">
          <n-el v-for="group in groupList" :key="group.id" class="min-w-40">
            <n-h5>{{ group.name }}</n-h5>
            <vue-draggable
              v-model="group.students"
              group="student"
              :animation="150"
              ghost-class="ghost"
              class="flex flex-col gap-2 p-4 bg-gray-500/5 rounded"
            >
              <n-el
                v-for="item in group.students"
                :key="item.id"
                class="cursor-move bg-gray-500/5 rounded p-3 cursor-move"
              >
                {{ item.name }}
              </n-el>
            </vue-draggable>
          </n-el>
        </n-el>
      </n-layout-content>

      <!-- 底部栏 -->
      <n-layout-footer bordered>
        <n-space class="px-3">拖拽来排序吧...</n-space>
      </n-layout-footer>
    </n-layout>

    <!-- 侧边栏 -->
    <n-layout-sider
      collapse-mode="width"
      :width="240"
      :collapsed-width="100"
      :native-scrollbar="true"
      show-trigger="arrow-circle"
      content-style="padding: 24px;"
      bordered
    >
      <n-el class="text-center">未分组</n-el>
      <vue-draggable
        v-model="ungroupStudentList"
        group="student"
        :animation="150"
        ghost-class="ghost"
        class="flex flex-col gap-2 p-4 bg-gray-500/5 rounded"
      >
        <n-el
          v-for="item in ungroupStudentList"
          :key="item.id"
          class="cursor-move bg-gray-500/5 rounded p-3 cursor-move"
        >
          {{ item.name }}
        </n-el>
      </vue-draggable>
    </n-layout-sider>
  </n-layout>
</template>

<style scoped>
.ghost {
  opacity: 0.5;
  background: #c8ebfb;
}
</style>
