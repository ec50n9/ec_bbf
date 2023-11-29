<script lang="ts" setup>
import { Student } from "@/api/student";
import { NThing } from "naive-ui";

defineProps<{
  student: Student;
  shrink?: boolean;
}>();
</script>

<template>
  <n-thing
    ref="itemRef"
    class="py-3 px-5 bg-white b rd-2 outline-2 outline-blue-3 cursor-pointer hover:shadow-lg"
  >
    <n-el tag="span" class="c-gray-4">{{ student.stu_no }}.</n-el>
    <n-el v-if="!shrink" tag="span" class="inline-block ml-1">{{
      student.name
    }}</n-el>
  </n-thing>
</template>
