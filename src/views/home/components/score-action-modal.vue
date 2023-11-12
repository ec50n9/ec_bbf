<script lang="ts" setup>
import { ScoreType } from "@/api/score";
import { Student } from "@/api/student";
import { FormInst } from "naive-ui";

export type Option = {
  action: "plus" | "minus";
  studentId: Student["id"];
  studentName: Student["name"];
  scoreTypeId: ScoreType["id"];
  scoreTypeName: ScoreType["name"];
  max: ScoreType["max"];
};

const emit = defineEmits<{
  (
    e: "submit",
    scoreTypeId: ScoreType["id"],
    value: number,
    reason: string
  ): void;
}>();

const state = ref<Option>({
  action: "plus",
  studentId: "",
  studentName: "",
  scoreTypeId: "",
  scoreTypeName: "",
  max: 0,
});

const actionText = computed(() =>
  state.value.action === "minus" ? "扣" : "加"
);
const title = computed(() => {
  return `给 ${state.value.studentName} ${actionText.value} ${state.value.scoreTypeName}`;
});

const formRef = ref<FormInst>();
const formValue = reactive({
  actionValue: 1,
  reason: "",
});

const formRules = computed<any>(() => ({
  actionValue: {
    required: true,
    type: "number",
    trigger: ["input", "blur"],
    message: `请输入${actionText.value}分分数`,
  },
  // reason: {
  //   required: true,
  //   trigger: ["input", "blur"],
  //   message: `请输入${actionText.value}分原因`,
  // },
}));

// 弹窗相关
const visible = ref(false);
const open = async (option: Option) => {
  visible.value = true;
  state.value = option;

  // 重置表单
  formValue.actionValue = 1;
  formValue.reason = "";
};
defineExpose({ open });
</script>

<template>
  <n-modal
    v-model:show="visible"
    class="w-3/5 max-w-100"
    preset="card"
    :title="title"
    size="medium"
    :bordered="false"
    :segmented="{
      content: false,
      footer: false,
    }"
  >
    <n-form
      ref="formRef"
      :label-width="80"
      :model="formValue"
      :rules="formRules"
      size="medium"
      label-placement="left"
    >
      <n-form-item :label="`${actionText}分分数`" path="actionValue">
        <n-input-number
          v-model:value="formValue.actionValue"
          :min="1"
          :max="state.max"
          :placeholder="`${actionText}多少分`"
        />
      </n-form-item>
      <n-form-item :label="`${actionText}分原因`" path="reason">
        <n-input
          v-model:value="formValue.reason"
          type="textarea"
          :placeholder="`${actionText}分原因`"
        />
      </n-form-item>
      <n-space justify="end">
        <n-button type="default" attr-type="button" @click="visible = false"
          >算了</n-button
        >
        <n-button
          :type="state.action === 'plus' ? 'primary' : 'error'"
          attr-type="button"
          @click="
            async () => {
              await formRef?.validate();
              emit(
                'submit',
                state.scoreTypeId,
                formValue.actionValue * (state.action === 'plus' ? 1 : -1),
                formValue.reason
              );
              visible = false;
            }
          "
        >
          确定{{ actionText }}分
        </n-button>
      </n-space>
    </n-form>
  </n-modal>
</template>
