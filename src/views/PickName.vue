<script lang="ts" setup>
import { computed, ref } from "vue";
import {
  NForm,
  NFormItem,
  FormInst,
  NInputNumber,
  NSwitch,
  NSpace,
  NButton,
  NIcon,
} from "naive-ui";
import { LocalFireDepartmentRound, AutorenewRound } from "@vicons/material";

const data = [
  {
    id: 1,
    label: "张三",
  },
  {
    id: 2,
    label: "李四",
  },
  {
    id: 3,
    label: "王五",
  },
];

/** 选中的下标列表 */
const selectedIndexList = ref<number[]>([1]);
/** 历史选中下标列表 */
const historyIndexList = ref<number[]>([1]);

/** 选中的数据列表 */
const selectedList = computed(() =>
  selectedIndexList.value.map((item) => data[item])
);

/** 配置表单 */
const formRef = ref<FormInst | null>(null);
const formValue = ref({
  /** 是否不重复抽取 */
  dontRepeat: false,
  /** 自动停止 */
  autoStop: false,
  /** 抽取人数 */
  quantity: 1,
});
const formRules = {
  quantity: [{ required: true, message: "请输入人数" }],
};

const createPicker = (watingToSelectList: number[], quantity: number) => {
  const localWatingToSelectList = [...watingToSelectList];

  const available = ref(false);
  const currentIndex = ref(0);
  const currentValue = computed(
    () => localWatingToSelectList[currentIndex.value]
  );
  let selectedCount = 0;
  let interval: number | null = null;

  const start = () => {
    if (interval) {
      clearInterval(interval);
      interval = null;
    }

    interval = setInterval(() => {
      if (currentIndex.value >= localWatingToSelectList.length - 1)
        currentIndex.value = 0;
      else currentIndex.value++;
    }, 80);
    available.value = true;
  };

  const stop = () => {
    if (interval) {
      clearInterval(interval);
      interval = null;
    }
    available.value = false;
  };

  const pick = () => {
    const value = currentValue.value;
    localWatingToSelectList.splice(currentIndex.value, 1);
    if (localWatingToSelectList.length === 0) stop();
    selectedCount++;
    if (selectedCount === quantity) stop();
    return value;
  };

  return {
    available,
    currentValue,
    start,
    pick,
    stop,
  };
};

const currentPicker = ref<any>(null);

/** 开始抽取 */
const handleStart = () => {
  // 验证表单
  formRef.value?.validate();

  // 获取配置
  const { dontRepeat, autoStop, quantity } = formValue.value;

  // 清空选中数组
  selectedIndexList.value = [];

  // 获取可选数组
  let watingToSelectList = data.map((_, index) => index);
  if (dontRepeat)
    watingToSelectList = watingToSelectList.filter(
      (index) => !historyIndexList.value.includes(index)
    );

  console.log(watingToSelectList);

  // 开始挑选
  currentPicker.value = createPicker(watingToSelectList, quantity);
  currentPicker.value.start();
};

const handleReset = () => {
  currentPicker.value = null;
  selectedIndexList.value = [];
  historyIndexList.value = [];
};

const handlePick = () => {
  const value = currentPicker.value.pick();
  selectedIndexList.value.push(value);
  historyIndexList.value.push(value);
  if (!currentPicker.value.available) currentPicker.value = null;
};
</script>

<template>
  <div>
    <div v-if="currentPicker" class="py-5 text-center text-5xl c-emerald-5">
      {{ data[currentPicker.currentValue].label }}
    </div>

    <!-- 名字展示 -->
    <ul class="px-5 py-10 flex justify-evenly gap-5 flex-wrap">
      <li v-for="item in selectedList" class="text-4xl c-blue-5">
        {{ item.label }}
      </li>
    </ul>

    <!-- 配置 -->
    <NForm
      ref="formRef"
      class="justify-center"
      inline
      size="medium"
      label-placement="left"
      label-width="auto"
      :model="formValue"
      :rules="formRules"
    >
      <NFormItem label="自动停止" path="autoStop">
        <NSwitch v-model:value="formValue.autoStop" />
      </NFormItem>

      <NFormItem label="不重复" path="dontRepeat">
        <NSwitch v-model:value="formValue.dontRepeat" />
      </NFormItem>

      <NFormItem label="人数" path="quantity">
        <NInputNumber
          class="w-25"
          v-model:value="formValue.quantity"
          placeholder="输入人数"
        />
      </NFormItem>
    </NForm>

    <!-- 操作 -->
    <NSpace class="mt-3" justify="center">
      <!-- 重置 -->
      <NButton size="large" round @click="handleReset">
        <template #icon>
          <NIcon>
            <AutorenewRound />
          </NIcon>
        </template>
        重置
      </NButton>

      <!-- 选择 -->
      <NButton
        v-if="currentPicker && currentPicker.available"
        type="info"
        size="large"
        round
        @click="handlePick"
      >
        <template #icon>
          <NIcon>
            <LocalFireDepartmentRound />
          </NIcon>
        </template>
        选择
      </NButton>

      <!-- 开始 -->
      <NButton v-else type="primary" size="large" round @click="handleStart">
        <template #icon>
          <NIcon>
            <LocalFireDepartmentRound />
          </NIcon>
        </template>
        开始！！！
      </NButton>
    </NSpace>
  </div>
</template>
