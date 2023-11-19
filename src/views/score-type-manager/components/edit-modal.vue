<script lang="ts" setup>
import {
  ScoreType,
  createScoreType,
  updateScoreType,
  getScoreTypeById,
  ScoreTypeUpdateVO,
} from "@/api/score";
import { FormInst } from "naive-ui";

const emit = defineEmits<{
  (e: "success"): void;
}>();

const modalVisible = ref(false);
const formRef = ref<FormInst>();

const mode = ref<"create" | "edit">("create");
const title = computed(
  () =>
    ({
      create: "添加",
      edit: "编辑",
    }[mode.value])
);

const open = async (id?: ScoreType["id"]) => {
  mode.value = id ? "edit" : "create";
  modalVisible.value = true;

  if (mode.value === "edit") {
    formValue.value = await getScoreTypeById(id!);
  } else {
    formValue.value = {
      name: "",
      desc: "",
      max: 100,
    };
  }
};
defineExpose({ open });

const formValue = ref<ScoreType>({
  name: "",
  desc: "",
  max: 100,
});
const formRules = {
  name: {
    required: true,
    message: "请输入名称",
    trigger: ["input", "blur"],
  },
  max: {
    type: "number",
    required: true,
    message: "请输入最大值",
    trigger: ["input", "blur"],
  },
} as any;

const handleSubmit = async () => {
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      if (mode.value === "create") {
        await createScoreType(formValue.value);
      } else if (mode.value === "edit") {
        await updateScoreType(formValue.value as ScoreTypeUpdateVO);
      }
      emit("success");
      modalVisible.value = false;
    }
  });
};
</script>

<template>
  <n-modal
    v-model:show="modalVisible"
    class="w-auto"
    preset="card"
    :title="title"
    @close="modalVisible = false"
  >
    <n-form ref="formRef" :model="formValue" :rules="formRules">
      <n-form-item label="名称" path="name">
        <n-input v-model:value="formValue.name" placeholder="输入名称" />
      </n-form-item>

      <n-form-item label="最大值" path="max">
        <n-input-number
          v-model:value="formValue.max"
          placeholder="输入最大值"
        />
      </n-form-item>

      <n-form-item label="描述" path="desc">
        <n-input
          v-model:value="formValue.desc"
          type="textarea"
          placeholder="输入描述"
        />
      </n-form-item>
    </n-form>

    <template #footer>
      <n-space justify="end">
        <n-button @click="modalVisible = false">取消</n-button>
        <n-button type="primary" @click="handleSubmit">确定</n-button>
      </n-space>
    </template>
  </n-modal>
</template>
