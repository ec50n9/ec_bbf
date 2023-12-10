<script lang="ts" setup>
import {
  RefreshRound as ResetIcon,
  PushPinTwotone as PinOffIcon,
  PinOffTwotone as PinOnIcon,
} from "@vicons/material";
import * as StudentApi from "@/api/modules/student";
import { Student, StudentCreateVO, StudentQueryVO } from "@/api/types/student";
import QueryBar from "./bars/query-bar.vue";
import PickNameBar from "./bars/pick-name-bar.vue";
import ToolsBar from "./bars/tools-bar.vue";
import UploadModal from "./modals/upload-modal.vue";
import StudentItem from "./components/student-item.vue";
import EditStudentFormModal from "./modals/edit-student-form-modal.vue";
import StudentScoreDetailsModal from "./modals/student-score-details.vue";
import { usePick } from "@/composables/pick";
import { useAppStore } from "@/store/modules/app";
import { useRequest } from "alova";

const message = useMessage();
const appStore = useAppStore();

let queryParams: StudentQueryVO = {};

const {
  data: studentList,
  loading: studentListLoading,
  send: fetchStudentList,
} = useRequest(() => StudentApi.getStudentList(queryParams), {
  immediate: true,
});

const studentIdMap = computed(
  () => new Map(studentList.value.map((student) => [student.id, student]))
);

// 查询列表
const handleQuery = async (params: StudentQueryVO) => {
  queryParams = params;
  await fetchStudentList();
};

// 编辑学生
const handleEditStudent = (id: Student["id"]) => {
  console.log("edit: ", id);
  editStudentFormModalRef.value?.open("update", id);
};

// 删除学生
const { send: deleteStudent } = useRequest(
  StudentApi.deleteStudent,
  { immediate: false }
);
const handleDeleteStudent = async (id: Student["id"]) => {
  await deleteStudent(id);
  message.success("删除成功");
  await fetchStudentList();
};

// 批量创建
const { send: batchCreateStudent } = useRequest(StudentApi.batchCreateStudent, { immediate: false });
const handleBatchCreateStudent = async (studentList: StudentCreateVO[]) => {
  await batchCreateStudent(studentList);
  message.success("批量添加成功");
  await fetchStudentList();
};

// 上传导入弹窗
const uploadModalRef = ref<typeof UploadModal>();
const openUploadModal = () => uploadModalRef.value?.open();

// 新建学生弹窗
const editStudentFormModalRef = ref<typeof EditStudentFormModal>();
const openCreateStudentFormModal = () =>
  editStudentFormModalRef.value?.open("create");

// 查看学生分数弹窗
const studentScoreDetailsModalRef = ref<typeof StudentScoreDetailsModal>();
const openStudentScoreDetailsModal = (id: Student["id"]) =>
  studentScoreDetailsModalRef.value?.open(id);

// 点名相关
const studentIds = computed(() =>
  studentList.value.map((student) => student.id)
);
const picker = usePick(studentIds);

const handleActiveTabChange = (_tab: string) => {
  picker.pause();
};
</script>

<template>
  <n-space class="p-3" vertical>
    <n-el class="px-5 pt-1 pb-3 b rd-3">
      <n-tabs type="line" @update:value="handleActiveTabChange" animated>
        <n-tab-pane name="query" tab="查找">
          <!-- 搜索框 -->
          <query-bar
            @query="handleQuery"
            @create="openCreateStudentFormModal"
            @import="openUploadModal"
          />
        </n-tab-pane>
        <n-tab-pane name="pick-name" tab="点名">
          <!-- 点名框 -->
          <pick-name-bar
            :pick-status="picker.status.value"
            @run="picker.run"
            @pause="picker.pause"
            @select="picker.select"
            @reset="picker.reset"
          />
        </n-tab-pane>
        <n-tab-pane name="settings" tab="小工具">
          <!-- 设置框 -->
          <tools-bar />
        </n-tab-pane>

        <template #suffix>
          <n-tooltip trigger="hover" :delay="500">
            <template #trigger>
              <n-button
                type="info"
                :dashed="!appStore.isAlwaysOnTop"
                :secondary="appStore.isAlwaysOnTop"
                size="small"
                circle
                @click="appStore.toggleAlwaysOnTop"
              >
                <template #icon>
                  <n-icon>
                    <pin-on-icon v-if="appStore.isAlwaysOnTop" />
                    <pin-off-icon v-else />
                  </n-icon>
                </template>
              </n-button>
            </template>
            {{ appStore.isAlwaysOnTop ? "取消置顶" : "置顶窗口" }}
          </n-tooltip>
        </template>
      </n-tabs>
    </n-el>

    <!-- <n-space align="center" class="mt-2">
      <n-radio-group name="radiobuttongroup1" size="small">
        <n-radio-button value="stu" label="按学生" />
        <n-radio-button value="group" label="按分组" />
      </n-radio-group>
    </n-space> -->

    <!-- 已选择的学生列表 -->
    <n-space v-if="picker.selectedList.value.size" align="center" class="mt-2">
      <n-el>已选择的学生:</n-el>
      <n-tag
        v-for="studentId in picker.selectedList.value"
        :key="studentId"
        type="info"
        @click="openStudentScoreDetailsModal(studentId)"
        closable
        @close="picker.unselect(studentId)"
      >
        {{ studentIdMap.get(studentId)?.name }}
      </n-tag>
      <n-tag type="error" @click="picker.reset()">
        <template #icon>
          <n-icon><reset-icon /></n-icon>
        </template>
        重置
      </n-tag>
    </n-space>

    <!-- 学生列表（flex布局 -->
    <n-spin :show="studentListLoading">
      <n-el
        v-if="studentList && studentList.length"
        class="mt-2 flex flex-wrap gap-3"
      >
        <student-item
          v-for="item in studentList"
          :key="item.id"
          :student="item"
          :focus="picker.currentFocusValue.value === item.id"
          :selected="picker.selectedList.value.has(item.id)"
          @detail="openStudentScoreDetailsModal"
        />
      </n-el>

      <n-empty v-else class="pt-10" description="什么都没有" />
    </n-spin>
  </n-space>

  <upload-modal ref="uploadModalRef" @upload="handleBatchCreateStudent" />
  <edit-student-form-modal
    ref="editStudentFormModalRef"
    @success="fetchStudentList"
  />
  <student-score-details-modal
    ref="studentScoreDetailsModalRef"
    @delete="handleDeleteStudent"
    @edit="handleEditStudent"
  />
</template>
