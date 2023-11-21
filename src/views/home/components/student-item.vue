<script lang="ts" setup>
import {
  PlusRound as PlusIcon,
} from "@vicons/material";
import { Student } from "@/api/student";
import { NThing } from "naive-ui";

defineProps<{
  student: Student;
  focus: boolean;
  selected: boolean;
}>();

const emit = defineEmits<{
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
