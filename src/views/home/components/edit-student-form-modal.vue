<script lang="ts" setup>
import { Student, StudentCreateVO, StudentUpdateVO } from "@/api/student";
import { FormInst, FormRules } from "naive-ui";
import { createStudent, updateStudent, getStudentById } from "@/api/student";

const emit = defineEmits<{
  (e: "success"): void;
}>();

const message = useMessage();

const visible = ref(false);

type OpenType = "create" | "update";
const openType = ref<OpenType>("create");
const modalTitle = computed(
  () =>
    ({
      create: "添加学生",
      update: "编辑学生",
    }[openType.value])
);

/** 打开弹窗 */
const open = async (type: OpenType = "create", id?: Student["id"]) => {
  openType.value = type;
  resetForm();
  visible.value = true;

  if (type === "update") {
    if (!id) throw Error("请传入需要编辑的学生id");
    model.value = await getStudentById(id);
  }
};
defineExpose({ open });

const model = ref<StudentCreateVO | StudentUpdateVO>({
  id: "",
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
      if (openType.value === "create") {
        const vo = model.value as StudentCreateVO;
        await createStudent(vo);
        message.success("添加成功");
      } else if (openType.value === "update") {
        const vo = model.value as StudentUpdateVO;
        await updateStudent(vo);
        message.success("修改成功");
      }
      visible.value = false;
      emit("success");
    }
  });
};
</script>

<template>
  <n-modal
    v-model:show="visible"
    class="w-4/5 max-w-120"
    preset="card"
    :title="modalTitle"
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
        <n-button type="primary" @click="handleSubmit">确定</n-button>
      </n-space>
    </template>
  </n-modal>
</template>
