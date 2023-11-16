<script lang="ts" setup>
import * as StudentApi from "@/api/student";
import QueryBar from "./bars/query-bar.vue";
import PickNameBar from "./bars/pick-name-bar.vue";
import SettingsBar from "./bars/settings-bar.vue";
import UploadModal from "./components/upload-modal.vue";
import StudentItem from "./components/student-item.vue";
import EditStudentFormModal from "./components/edit-student-form-modal.vue";
import StudentScoreDetailsModal from "./components/student-score-details-modal.vue";

const message = useMessage();

let queryParams: StudentApi.StudentQueryVO = {};

const studentList = ref<StudentApi.Student[]>([]);
const getStudentList = async () => {
  studentList.value = await StudentApi.getStudentList(queryParams);
};

// 查询列表
const handleQuery = async (params: StudentApi.StudentQueryVO) => {
  queryParams = params;
  await getStudentList();
};

// 编辑学生
const handleEditStudent = (id: StudentApi.Student["id"]) =>
  editStudentFormModalRef.value?.open("update", id);

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

// 初始化
getStudentList();
</script>

<template>
  <n-space class="p-3" vertical>
    <n-el class="px-5 pt-1 pb-3 b rd-3">
      <n-tabs type="line" animated>
        <n-tab-pane name="query" tab="查找">
          <!-- 搜索框 -->
          <query-bar @query="handleQuery" @create="openCreateStudentFormModal" @import="openUploadModal" />
        </n-tab-pane>
        <n-tab-pane name="pick-name" tab="点名">
          <!-- 点名框 -->
          <pick-name-bar />
        </n-tab-pane>
        <n-tab-pane name="settings" tab="设置">
          <!-- 设置框 -->
          <settings-bar />
        </n-tab-pane>
      </n-tabs>
    </n-el>

    <!-- 学生列表 -->
    <n-grid class="mt-2" x-gap="12" y-gap="12" :cols="4">
      <n-gi v-for="item in studentList" :key="item.id">
        <student-item :student="item" @delete="handleDeleteStudent" @edit="handleEditStudent"
          @detail="openStudentScoreDetailsModal" />
      </n-gi>
    </n-grid>
  </n-space>

  <upload-modal ref="uploadModalRef" @upload="handleBatchCreateStudent" />
  <edit-student-form-modal ref="editStudentFormModalRef" @success="getStudentList" />
  <student-score-details-modal ref="studentScoreDetailsModalRef" />
</template>
