<script lang="ts" setup>
import {
  DeleteOutlineRound as DeleteIcon,
  PlusRound as PlusIcon,
  ModeOutlined as EditIcon,
} from "@vicons/material";
import { Student } from "@/api/student";
import { NThing } from "naive-ui";

defineProps<{
  student: Student;
}>();

const emit = defineEmits<{
  (e: "delete", id: Student["id"]): void;
  (e: "edit", id: Student["id"]): void;
}>();

// 鼠标移入悬浮
const itemRef = ref<typeof NThing>();
const hovering = ref(false);
const itemWidth = ref(0);

const handleMouseEnter = () => {
  const el = itemRef.value?.$el as HTMLDivElement | undefined;
  if (!el) return;

  itemWidth.value = el.getBoundingClientRect().width;
  hovering.value = true;
};
const handleMouseLeave = () => {
  hovering.value = false;
};
</script>

<template>
  <n-thing
    ref="itemRef"
    class="p-3 bg-white b rd-2 transition duration-300"
    :class="{ 'shadow-lg absolute': hovering }"
    :style="{ width: hovering ? itemWidth + 'px' : 'auto' }"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
  >
    <template #header>{{ student.name }}</template>
    <template #header-extra>
      <n-space size="small">
        <!-- 编辑 -->
        <n-button
          secondary
          circle
          size="small"
          type="warning"
          @click="emit('edit', student.id)"
        >
          <template #icon>
            <n-icon><edit-icon /></n-icon>
          </template>
        </n-button>
        <!-- 删除 -->
        <n-popconfirm
          negative-text="取消"
          positive-text="确定"
          @positive-click="emit('delete', student.id)"
        >
          <template #trigger>
            <n-button secondary circle size="small" type="error">
              <template #icon>
                <n-icon><delete-icon /></n-icon>
              </template>
            </n-button>
          </template>
          删除了就没有了喔
        </n-popconfirm>
      </n-space>
    </template>
    <template #description>{{ student.stu_no }}</template>
    <template #action>
      <n-collapse-transition :show="hovering">
        <n-space>
          <n-button size="small" type="primary">
            <template #icon>
              <n-icon><plus-icon /></n-icon>
            </template>
            加分
          </n-button>
        </n-space>
      </n-collapse-transition>
    </template>
  </n-thing>
</template>
