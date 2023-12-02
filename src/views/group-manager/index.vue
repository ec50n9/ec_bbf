<script lang="ts" setup>
import { VueDraggable } from "vue-draggable-plus";
import StudentItem from "./components/student-item.vue";
import EditModal from "./components/edit-modal.vue";
import {
  getStudentGroupList,
  getAllStudentGroupMapping,
  batchUpdateStudentGroupRel,
  StudentGroup,
  StudentWithGroupIdVO,
  StudentGroupBindVO,
  StudentGroupUnbindVO,
  StudentGroupUpdateRelVO,
} from "@/api/student-group";

/** 已分组 */
const groupList = ref<(StudentGroup & { students: StudentWithGroupIdVO[] })[]>(
  []
);

/** 未分组 */
const ungroupStudentList = ref<StudentWithGroupIdVO[]>([]);

const handleSave = async () => {
  const needBindList: StudentGroupBindVO[] = [];
  const needUnbindList: StudentGroupUnbindVO[] = [];
  const needUpdateRelList: StudentGroupUpdateRelVO[] = [];

  const groupListValue = unref(groupList);
  const ungroupStudentListValue = unref(ungroupStudentList);
  console.log("分组：", unref(groupList));
  console.log("未分组：", unref(ungroupStudentList));

  groupListValue.forEach((group) => {
    group.students.forEach((student) => {
      // 新增绑定
      if (student.group_id === null) {
        needBindList.push({
          student_id: student.student_id,
          group_id: group.id!,
        });
      }
      // 更改绑定
      else if (student.group_id !== group.id) {
        needUpdateRelList.push({
          student_id: student.student_id,
          update_to_group_id: group.id!,
        });
      }
    });
  });

  ungroupStudentListValue.forEach((student) => {
    // 解绑
    if (student.group_id !== null) {
      needUnbindList.push({
        student_id: student.student_id,
      });
    }
  });

  console.log("needBindList", needBindList);
  console.log("needUnbindList", needUnbindList);
  console.log("needUpdateList", needUpdateRelList);

  const res = await batchUpdateStudentGroupRel({
    needBindList,
    needUnbindList,
    needUpdateRelList,
  });
  console.log("更新完成：", res);

  init();
};

const editModalRef = ref<typeof EditModal>();
const open = (id?: string) => editModalRef.value?.open(id);

const init = async () => {
  console.log("init...");
  const allGroupList = await getStudentGroupList();
  const studentWithGroupIdList = await getAllStudentGroupMapping();

  const groupStudentMap = studentWithGroupIdList.reduce((prev, curr) => {
    const key = curr.group_id || "ungroup";

    if (!prev.has(key)) prev.set(key, []);
    prev.get(key)!.push(curr);
    return prev;
  }, new Map<StudentGroup["id"], StudentWithGroupIdVO[]>());

  // 已分组
  groupList.value = allGroupList.map((group) => ({
    ...group,
    students: groupStudentMap.get(group.id) || [],
  }));
  // 未分组
  ungroupStudentList.value = groupStudentMap.get("ungroup") || [];
};

init();
</script>

<template>
  <n-layout has-sider sider-placement="right" class="h-full">
    <n-layout content-style="display: flex; flex-direction: column">
      <!-- 标题栏 -->
      <n-layout-header bordered>
        <n-space class="p-3">
          <n-button type="primary" @click="open()">添加</n-button>
          <n-button @click="handleSave">保存</n-button>
        </n-space>
      </n-layout-header>

      <!-- 分组列表 -->
      <n-layout-content content-style="padding: 24px;">
        <n-el
          class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 2xl:grid-cols-8 gap-3"
        >
          <n-el v-for="group in groupList" :key="group.id" class="min-w-40">
            <n-el class="c-gray-4">{{ group.name }}</n-el>
            <vue-draggable
              v-model="group.students"
              group="student"
              :animation="150"
              ghost-class="ghost"
              class="flex flex-col gap-2 p-3 bg-gray-1 rd-4"
            >
              <student-item
                v-for="item in group.students"
                :key="item.student_id"
                :student="item"
              />
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
      bordered
    >
      <n-el class="h-full flex flex-col">
        <n-el class="shrink-0 my-3 text-lg text-center">未分组</n-el>
        <vue-draggable
          v-model="ungroupStudentList"
          group="student"
          :animation="150"
          ghost-class="ghost"
          class="basis-0 grow of-auto flex flex-col gap-2 p-3 bg-gray-1 rounded"
        >
          <student-item
            v-for="item in ungroupStudentList"
            :key="item.student_id"
            :student="item"
          />
        </vue-draggable>
      </n-el>
    </n-layout-sider>

    <edit-modal ref="editModalRef" @success="init" />
  </n-layout>
</template>

<style scoped>
.ghost {
  opacity: 0.5;
  background: #c8ebfb;
}
</style>
