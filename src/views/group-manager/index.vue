<script lang="ts" setup>
import { VueDraggable } from "vue-draggable-plus";
import StudentItem from "./components/student-item.vue";
import EditModal from "./components/edit-modal.vue";
import * as StudentGroupApi from "@/api/modules/student-group";
import {
  CancelOutlined as CancelIcon,
  PlusRound as PlusIcon,
  SaveOutlined as SaveIcon,
} from "@vicons/material";
import {
  StudentGroup,
  StudentGroupBindVO,
  StudentGroupUnbindVO,
  StudentGroupUpdateRelVO,
  StudentWithGroupIdVO,
} from "@/api/types/student-group";
import { useRequest } from "alova";

const message = useMessage();

/** 已分组 */
const groupList = ref<(StudentGroup & { students: StudentWithGroupIdVO[] })[]>(
  []
);

/** 未分组 */
const ungroupStudentList = ref<StudentWithGroupIdVO[]>([]);

const { send: batchUpdateStudentGroupRel } = useRequest(
  StudentGroupApi.batchUpdateStudentGroupRel,
  {
    immediate: false,
  }
);

const {send: deleteStudentGroup} = useRequest(StudentGroupApi.deleteStudentGroup, {
  immediate: false,
});

const {send: getStudentGroupList} = useRequest(StudentGroupApi.getStudentGroupList, {
  immediate: false,
});

const {send: getAllStudentGroupMapping} = useRequest(StudentGroupApi.getAllStudentGroupMapping, {
  immediate: false,
});

/** 保存分组关系 */
const handleSave = async () => {
  const needBindList: StudentGroupBindVO[] = [];
  const needUnbindList: StudentGroupUnbindVO[] = [];
  const needUpdateRelList: StudentGroupUpdateRelVO[] = [];

  const groupListValue = unref(groupList);
  const ungroupStudentListValue = unref(ungroupStudentList);

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

  await batchUpdateStudentGroupRel({
    needBindList,
    needUnbindList,
    needUpdateRelList,
  });

  message.success("保存成功");
};

const editModalRef = ref<typeof EditModal>();
const open = (id?: string) => editModalRef.value?.open(id);

/** 删除分组 */
const handleDelete = async (groupId: string) => {
  const group = groupList.value.find((group) => group.id === groupId);
  if (group!.students.length > 0) {
    message.error("该分组下有学生无法删除");
    return;
  }

  await deleteStudentGroup(groupId);
  init();
};

/** 学生被右键点击 */
const handleStudentItemRightClick = (
  students: StudentWithGroupIdVO[],
  student: StudentWithGroupIdVO
) => {
  students.splice(students.indexOf(student), 1);
  ungroupStudentList.value.unshift(student);
};

const init = async () => {
  await handleSave();
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
          <n-button secondary type="info" round @click="open()">
            <template #icon>
              <n-icon><plus-icon /></n-icon>
            </template>
            添加
          </n-button>
          <n-button secondary type="primary" round @click="init()">
            <template #icon>
              <n-icon><save-icon /></n-icon>
            </template>
            保存
          </n-button>
        </n-space>
      </n-layout-header>

      <!-- 分组列表 -->
      <n-layout-content content-style="padding: 24px;">
        <n-el
          v-if="groupList.length"
          class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 2xl:grid-cols-8 gap-3"
        >
          <n-el v-for="group in groupList" :key="group.id" class="min-w-40">
            <n-el class="flex justify-between items-center">
              <n-el class="c-gray-4" @click="open(group.id)">{{
                group.name
              }}</n-el>
              <!-- 删除分组 -->
              <n-popconfirm
                negative-text="取消"
                positive-text="确认"
                @positive-click="handleDelete(group.id!)"
              >
                <template #trigger>
                  <n-button class="c-gray-3" size="small" quaternary circle>
                    <template #icon>
                      <n-icon><cancel-icon /></n-icon>
                    </template>
                  </n-button>
                </template>
                删除了就没有了哦
              </n-popconfirm>
            </n-el>
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
                @contextmenu.prevent="
                  handleStudentItemRightClick(group.students, item)
                "
              />
            </vue-draggable>
          </n-el>
        </n-el>

        <n-empty v-else description="一个分组都没有呢">
          <template #extra>
            <n-button size="small" @click="open()">创建一个</n-button>
          </template>
        </n-empty>
      </n-layout-content>

      <!-- 底部栏 -->
      <n-layout-footer bordered>
        <n-space class="px-3"
          >拖拽以排序、右键以删除、点击分组名称以修改</n-space
        >
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
