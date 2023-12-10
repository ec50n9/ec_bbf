<script lang="ts" setup>
import { ScoreType } from "@/api/types/score";
import { Student } from "@/api/types/student";
import { useAppStore } from "@/store/modules/app";
import { FormInst } from "naive-ui";

export type Option = {
  action: "plus" | "minus";
  studentId: Student["id"];
  studentName: Student["name"];
  scoreTypeId: ScoreType["id"];
  scoreTypeName: ScoreType["name"];
  currentScore: number;
};

const appStore = useAppStore();

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
  currentScore: 0,
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
    class="w-4/5 max-w-120"
    preset="card"
    :title="title"
    :size="appStore.miniWindowMode ? 'small' : 'medium'"
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
      :size="appStore.miniWindowMode ? 'small' : 'medium'"
      label-placement="left"
    >
      <n-grid :cols="24" :x-gap="24">
        <n-form-item-gi :span="24" label="操作">
          <n-radio-group v-model:value="state.action" size="small">
            <n-radio-button value="plus">加分</n-radio-button>
            <n-radio-button value="minus">扣分</n-radio-button>
          </n-radio-group>
        </n-form-item-gi>

        <n-form-item-gi
          :span="24"
          :label="`${actionText}分分数`"
          path="actionValue"
        >
          <n-input-number
            v-model:value="formValue.actionValue"
            :min="1"
            :placeholder="`${actionText}多少分`"
          />
        </n-form-item-gi>

        <!-- <n-form-item-gi :span="24" :label="`${actionText}分原因`" path="reason">
          <n-input
            v-model:value="formValue.reason"
            type="textarea"
            :placeholder="`${actionText}分原因`"
          />
        </n-form-item-gi> -->
      </n-grid>

      <n-space justify="end">
        <n-button
          type="default"
          :size="appStore.miniWindowMode ? 'small' : 'medium'"
          attr-type="button"
          @click="visible = false"
          >算了</n-button
        >
        <n-button
          :type="state.action === 'plus' ? 'primary' : 'error'"
          :size="appStore.miniWindowMode ? 'small' : 'medium'"
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
