<script lang="ts" setup>
import { StudentCreateVO } from "@/api/student";
import { FormInst, FormRules } from "naive-ui";
import { createStudent } from "@/api/student";

const emit = defineEmits<{
  (e: "success"): void;
}>();

const visible = ref(false);

const message = useMessage();

/** 打开弹窗 */
const open = () => {
  visible.value = true;
};
defineExpose({ open });

const model = ref<StudentCreateVO>({
  name: "",
  stu_no: "",
});
const rules: FormRules = {
  name: [{ required: true, message: "请输入姓名" }],
  stu_no: [{ required: true, message: "请输入学号" }],
};

const formRef = ref<FormInst>();

const resetForm = () => {
  model.value = {
    name: "",
    stu_no: "",
  };
};

const handleSubmit = () => {
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      await createStudent(model.value);
      resetForm();
      visible.value = false;
      message.success("添加成功");
      emit("success");
    }
  });
};
</script>

<template>
  <!-- 上传数据 -->
  <n-modal
    v-model:show="visible"
    class="w-4/5 max-w-120"
    preset="card"
    title="添加学生"
    size="medium"
    :bordered="false"
    :segmented="{
      content: false,
      footer: false,
    }"
  >
    <n-form
      ref="formRef"
      :model="model"
      :rules="rules"
      class="max-w-80 mx-auto"
      label-width="auto"
      label-placement="left"
    >
      <n-form-item path="name" label="姓名">
        <n-input
          v-model:value="model.name"
          placeholder="请输入姓名"
          @keydown.enter.prevent
        />
      </n-form-item>
      <n-form-item path="stu_no" label="学号">
        <n-input
          v-model:value="model.stu_no"
          placeholder="请输入学号"
          @keydown.enter.prevent
        />
      </n-form-item>
    </n-form>

    <template #footer>
      <n-space justify="end">
        <n-button @click="visible = false">取消</n-button>
        <n-button type="primary" @click="handleSubmit">添加</n-button>
      </n-space>
    </template>
  </n-modal>
</template>
