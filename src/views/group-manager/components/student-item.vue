<script lang="ts" setup>
import { StudentWithGroupIdVO } from "@/api/types/student-group";
import { NThing } from "naive-ui";

defineProps<{
  student: StudentWithGroupIdVO;
  shrink?: boolean;
}>();
</script>

<template>
  <n-thing
    ref="itemRef"
    class="py-3 px-5 bg-white b rd-2 outline-2 outline-blue-3 select-none cursor-pointer hover:shadow-lg"
  >
    <n-el tag="span" class="c-gray-4">{{ student.student_no }}.</n-el>
    <n-el v-if="!shrink" tag="span" class="inline-block ml-1">{{
      student.student_name
    }}</n-el>
  </n-thing>
</template>
