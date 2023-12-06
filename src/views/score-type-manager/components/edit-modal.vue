<script lang="ts" setup>
import { ScoreType, ScoreTypeUpdateVO } from "@/apis/types/score";
import {
  createScoreType,
  updateScoreType,
  getScoreTypeById,
} from "@/apis/modules/score";
import { useRequest } from "alova";
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

const {
  data: formValue,
  loading,
  send: fetchScoreType,
} = useRequest(getScoreTypeById, { immediate: false });

const open = async (id?: ScoreType["id"]) => {
  mode.value = id ? "edit" : "create";
  modalVisible.value = true;

  formValue.value = {
    name: "",
    desc: "",
  };
  if (mode.value === "edit") fetchScoreType(id!);
};
defineExpose({ open });

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

const {
  send: submit,
  loading: submitLoading,
  onSuccess: onSubmitSuccess,
} = useRequest(
  () => {
    let res: any = null;
    if (mode.value === "edit") {
      res = updateScoreType(formValue.value as ScoreTypeUpdateVO);
    } else {
      res = createScoreType(formValue.value);
    }
    return res;
  },
  { immediate: false }
);

onSubmitSuccess(() => {
  modalVisible.value = false;
  emit("success");
});

const handleSubmit = async () => {
  formRef.value?.validate(async (errors) => {
    if (!errors) submit();
  });
};
</script>

<template>
  <n-modal
    v-model:show="modalVisible"
    class="w-4/5 max-w-100"
    preset="card"
    :title="title"
    @close="modalVisible = false"
  >
    <n-spin :show="loading">
      <n-form ref="formRef" :model="formValue" :rules="formRules">
        <n-form-item label="名称" path="name">
          <n-input v-model:value="formValue.name" placeholder="输入名称" />
        </n-form-item>

        <n-form-item label="描述" path="desc">
          <n-input
            v-model:value="formValue.desc"
            type="textarea"
            placeholder="输入描述"
          />
        </n-form-item>

        <n-form-item label="图标" path="icon">
          <n-input v-model:value="formValue.icon" placeholder="输入描述" />
        </n-form-item>

        <n-form-item label="颜色" path="color">
          <n-color-picker
            v-model:value="formValue.color"
            :swatches="[
              '#FFFFFF',
              '#18A058',
              '#2080F0',
              '#F0A020',
              'rgba(208, 48, 80, 1)',
            ]"
          />
        </n-form-item>
      </n-form>
    </n-spin>

    <template #footer>
      <n-space justify="end">
        <n-button @click="modalVisible = false">取消</n-button>
        <n-button type="primary" @click="handleSubmit" :loading="submitLoading"
          >确定</n-button
        >
      </n-space>
    </template>
  </n-modal>
</template>
