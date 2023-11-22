<script lang="ts" setup>
import { RefreshRound as ResetIcon } from "@vicons/material";
import * as StudentApi from "@/api/student";
import QueryBar from "./bars/query-bar.vue";
import PickNameBar from "./bars/pick-name-bar.vue";
import ToolsBar from "./bars/tools-bar.vue";
import UploadModal from "./components/upload-modal.vue";
import StudentItem from "./components/student-item.vue";
import EditStudentFormModal from "./components/edit-student-form-modal.vue";
import StudentScoreDetailsModal from "./components/student-score-details-modal.vue";
import { usePick } from "@/composables/pick";

const message = useMessage();

let queryParams: StudentApi.StudentQueryVO = {};

const studentList = ref<StudentApi.Student[]>([]);
const studentIdMap = computed(
  () => new Map(studentList.value.map((student) => [student.id, student]))
);

const getStudentList = async () => {
  studentList.value = await StudentApi.getStudentList(queryParams);
};

// 查询列表
const handleQuery = async (params: StudentApi.StudentQueryVO) => {
  queryParams = params;
  await getStudentList();
};

// 编辑学生
const handleEditStudent = (id: StudentApi.Student["id"]) => {
  console.log("edit: ", id);
  editStudentFormModalRef.value?.open("update", id);
};

// 删除学生
const handleDeleteStudent = async (id: StudentApi.Student["id"]) => {
  await StudentApi.deleteStudent(id);
  message.success("删除成功");
  await getStudentList();
};

// 批量创建
const handleBatchCreateStudent = async (
  studentList: StudentApi.StudentCreateVO[]
) => {
  await StudentApi.batchCreateStudent(studentList);
  message.success("批量添加成功");
  await getStudentList();
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
const openStudentScoreDetailsModal = (id: StudentApi.Student["id"]) =>
  studentScoreDetailsModalRef.value?.open(id);

// 点名相关
const studentIds = computed(() =>
  studentList.value.map((student) => student.id)
);
const picker = usePick(studentIds);

const handleActiveTabChange = (_tab: string) => {
  picker.pause();
};

// 初始化
getStudentList();
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

    <!-- 学生列表（网格布局 -->
    <!-- <n-grid class="mt-2" x-gap="12" y-gap="12" cols="2 s:4 m:5 l:6 xl:7 2xl:8" responsive="screen">
      <n-gi v-for="item in studentList" :key="item.id">
        <student-item
          :student="item"
          :focus="picker.currentFocusValue.value === item.id"
          :selected="picker.selectedList.value.has(item.id)"
          @detail="openStudentScoreDetailsModal"
        />
      </n-gi>
    </n-grid> -->

    <!-- 学生列表（flex布局 -->
    <n-el class="mt-2 flex flex-wrap gap-3">
      <student-item
        v-for="item in studentList"
        :key="item.id"
        :student="item"
        :focus="picker.currentFocusValue.value === item.id"
        :selected="picker.selectedList.value.has(item.id)"
        @detail="openStudentScoreDetailsModal"
      />
    </n-el>
  </n-space>

  <upload-modal ref="uploadModalRef" @upload="handleBatchCreateStudent" />
  <edit-student-form-modal
    ref="editStudentFormModalRef"
    @success="getStudentList"
  />
  <student-score-details-modal
    ref="studentScoreDetailsModalRef"
    @delete="handleDeleteStudent"
    @edit="handleEditStudent"
  />
</template>
