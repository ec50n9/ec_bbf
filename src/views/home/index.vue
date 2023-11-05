<script lang="ts" setup>
import * as StudentApi from "@/api/student";
import UploadModal from "./components/upload-modal.vue";
import StudentItem from "./components/student-item.vue";
import CreateStudentForm from "./components/create-student-form.vue";
import { FormInst } from "naive-ui";

const message = useMessage();

const studentList = ref<StudentApi.Student[]>([]);
const getStudentList = async (query: StudentApi.StudentQueryVO = {}) => {
  console.log("hello");
  studentList.value = await StudentApi.getStudentList(query);
};

/** 搜索表单 */
const queryFormValue = ref<StudentApi.StudentQueryVO>({
  name: "",
  stu_no: "",
});
const queryFormRules = {};
const queryFormRef = ref<FormInst>();
/** 验证表单 */
const validateQueryForm = () =>
  queryFormRef.value?.validate((errors) => {
    if (!errors) {
      message.success("Valid");
    } else {
      console.log(errors);
      message.error("Invalid");
    }
  });

const validateQueryFormAndSearch = async () => {
  await validateQueryForm();
  await getStudentList(queryFormValue.value);
};

// 上传导入弹窗
const uploadModalRef = ref<typeof UploadModal>();
const openUploadModal = () => uploadModalRef.value?.open();

// 新建学生弹窗
const createStudentFormRef = ref<typeof UploadModal>();
const openCreateStudentForm = () => createStudentFormRef.value?.open();

// 初始化
getStudentList();
</script>

<template>
  <n-space class="p-3" vertical>
    <n-space class="p-3 bg-white b rd-2">
      <n-form
        ref="queryFormRef"
        inline
        :label-width="40"
        :model="queryFormValue"
        :rules="queryFormRules"
        size="small"
        label-placement="left"
      >
        <n-form-item label="学号" path="stu_no">
          <n-input
            v-model:value="queryFormValue.stu_no"
            placeholder="请输入学号"
          />
        </n-form-item>
        <n-form-item label="姓名" path="name">
          <n-input
            v-model:value="queryFormValue.name"
            placeholder="请输入姓名"
          />
        </n-form-item>
        <n-form-item>
          <n-button attr-type="button" @click="validateQueryFormAndSearch">
            搜索
          </n-button>
        </n-form-item>
        <n-form-item>
          <n-button
            attr-type="button"
            type="primary"
            @click="openCreateStudentForm"
          >
            添加
          </n-button>
        </n-form-item>
        <n-form-item>
          <n-button attr-type="button" type="info" @click="openUploadModal">
            导入
          </n-button>
        </n-form-item>
      </n-form>
    </n-space>

    <n-grid class="mt-2" x-gap="12" y-gap="12" :cols="4">
      <n-gi v-for="item in studentList" :key="item.id">
        <student-item
          :student="item"
          @refresh="getStudentList(queryFormValue)"
        />
      </n-gi>
    </n-grid>
  </n-space>

  <upload-modal ref="uploadModalRef" />
  <create-student-form
    ref="createStudentFormRef"
    @success="getStudentList(queryFormValue)"
  />
</template>
