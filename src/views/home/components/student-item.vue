<script lang="ts" setup>
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
    class="py-3 px-5 bg-white b rd-2 outline-2 outline-blue-3 cursor-pointer transition duration-300 hover:shadow-lg"
    :class="{ 'bg-blue-1!': selected, 'outline bg-blue-1!': focus }"
    @click="emit('detail', student.id)"
  >
    <n-el tag="span" class="c-gray-4">{{ student.stu_no }}.</n-el>
    <n-el tag="span" class="inline-block ml-1">{{ student.name }}</n-el>
  </n-thing>
</template>
