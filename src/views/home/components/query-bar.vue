<script lang="ts" setup>
import * as StudentApi from "@/api/student";
import * as ScoreApi from "@/api/score";
import {
  PlusRound as PlusIcon,
  FileUploadOutlined as UploadIcon,
} from "@vicons/material";
import { FormInst } from "naive-ui";

const emit = defineEmits<{
  (e: "query", params: StudentApi.StudentQueryVO): void;
  (e: "create"): void;
  (e: "import"): void;
}>();

/** 搜索表单 */
const queryFormValue = ref<StudentApi.StudentQueryVO>({
  name: "",
  stu_no: "",
});
const queryFormRules = {};
const queryFormRef = ref<FormInst>();
/** 验证表单 */
const validateQueryForm = () => queryFormRef.value?.validate();

const validateQueryFormAndSearch = async () => {
  await validateQueryForm();
  emit("query", unref(queryFormValue));
};

const handleTest = async () => {
  const scoreTypeList = await ScoreApi.getScoreTypeList();
  console.log(scoreTypeList);
};
</script>

<template>
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
        <n-input v-model:value="queryFormValue.name" placeholder="请输入姓名" />
      </n-form-item>

      <n-form-item>
        <n-button attr-type="button" @click="validateQueryFormAndSearch">
          搜索
        </n-button>
      </n-form-item>

      <n-form-item>
        <n-button attr-type="button" type="primary" @click="emit('create')">
          <template #icon>
            <n-icon><plus-icon /></n-icon>
          </template>
          添加
        </n-button>
      </n-form-item>

      <n-form-item>
        <n-button attr-type="button" type="info" @click="emit('import')">
          <template #icon>
            <n-icon><upload-icon /></n-icon>
          </template>
          导入
        </n-button>
      </n-form-item>

      <n-form-item>
        <n-button attr-type="button" type="info" @click="handleTest">
          <template #icon>
            <n-icon><upload-icon /></n-icon>
          </template>
          测试
        </n-button>
      </n-form-item>
    </n-form>
  </n-space>
</template>
