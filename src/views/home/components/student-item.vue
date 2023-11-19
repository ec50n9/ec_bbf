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
  focus: boolean;
  selected: boolean;
}>();

const emit = defineEmits<{
  (e: "delete", id: Student["id"]): void;
  (e: "edit", id: Student["id"]): void;
  (e: "detail", id: Student["id"]): void;
}>();
</script>

<template>
  <n-thing
    ref="itemRef"
    class="p-3 bg-white b rd-2 outline-2 outline-blue-3 cursor-pointer transition duration-300 hover:shadow-lg"
    :class="{ 'bg-blue-1!': selected, 'outline bg-blue-1!': focus }"
    @click="emit('detail', student.id)"
  >
    <template #header>
      <n-el>{{ student.name }}</n-el>
    </template>
    <template #header-extra>
      <n-el>
        <!-- 编辑 -->
        <n-button
        quaternary
          circle
          size="small"
          type="warning"
          @click.stop="emit('edit', student.id)"
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
            <n-button quaternary circle size="small" type="error" @click.stop>
              <template #icon>
                <n-icon><delete-icon /></n-icon>
              </template>
            </n-button>
          </template>
          删除了就没有了喔
        </n-popconfirm>
      </n-el>
    </template>
    <template #description>{{ student.stu_no }}</template>
    <template v-if="false" #action>
      <n-space>
        <n-button strong secondary size="small" type="primary" round>
          <template #icon>
            <n-icon><plus-icon /></n-icon>
          </template>
          加分
        </n-button>
      </n-space>
    </template>
  </n-thing>
</template>
