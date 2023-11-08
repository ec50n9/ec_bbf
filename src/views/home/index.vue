<script lang="ts" setup>
import * as StudentApi from "@/api/student";
import QueryBar from "./components/query-bar.vue";
import UploadModal from "./components/upload-modal.vue";
import StudentItem from "./components/student-item.vue";
import EditStudentForm from "./components/edit-student-form.vue";

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
  editStudentFormRef.value?.open("update", id);

// 删除学生
const handleDeleteStudent = async (id: StudentApi.Student["id"]) => {
  await StudentApi.deleteStudent(id);
  message.success("删除成功");
  await getStudentList();
};

// 查看学生详情
const handleViewStudentDetail = async (id: StudentApi.Student["id"]) => {
  message.success("查看详情");
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
const editStudentFormRef = ref<typeof UploadModal>();
const openCreateStudentForm = () => editStudentFormRef.value?.open("create");

// 初始化
getStudentList();
</script>

<template>
  <n-space class="p-3" vertical>
    <!-- 搜索框 -->
    <query-bar
      @query="handleQuery"
      @create="openCreateStudentForm"
      @import="openUploadModal"
    />

    <!-- 学生列表 -->
    <n-grid class="mt-2" x-gap="12" y-gap="12" :cols="4">
      <n-gi v-for="item in studentList" :key="item.id">
        <student-item
          :student="item"
          @delete="handleDeleteStudent"
          @edit="handleEditStudent"
          @detail="handleViewStudentDetail"
        />
      </n-gi>
    </n-grid>
  </n-space>

  <upload-modal ref="uploadModalRef" @upload="handleBatchCreateStudent" />
  <edit-student-form ref="editStudentFormRef" @success="getStudentList" />
</template>
