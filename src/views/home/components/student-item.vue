<script lang="ts" setup>
import {
  DeleteOutlineRound as DeleteIcon,
  PlusRound as PlusIcon,
  EditRound as EditIcon,
} from "@vicons/material";
import { Student, deleteStudent } from "@/api/student";
import { NThing } from "naive-ui";

defineProps<{
  student: Student;
}>();

const emit = defineEmits<{
  (e: "refresh"): void;
}>();

const message = useMessage();

// 删除学生
const handleDelete = async (id: Student["id"]) => {
  await deleteStudent(id);
  message.success("删除成功");
  emit("refresh");
};

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
    <template v-if="hovering" #header-extra>
      <n-space size="small">
        <!-- 编辑 -->
        <n-button quaternary circle size="small">
          <template #icon>
            <n-icon><edit-icon /></n-icon>
          </template>
        </n-button>
        <!-- 删除 -->
        <n-button
          quaternary
          circle
          size="small"
          type="error"
          @click="handleDelete(student.id)"
        >
          <template #icon>
            <n-icon><delete-icon /></n-icon>
          </template>
        </n-button>
      </n-space>
    </template>
    <template #description>{{ student.stu_no }}</template>
    <template v-if="hovering" #action>
      <n-button size="small" type="primary">
        <template #icon>
          <n-icon><plus-icon /></n-icon>
        </template>
        加分
      </n-button>
    </template>
  </n-thing>
</template>
