<script lang="ts" setup>
import { Student, StudentWithScore } from "@/api/types/student";
import { NThing } from "naive-ui";

defineProps<{
  student: StudentWithScore;
  focus?: boolean;
  selected?: boolean;
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

    <template #action>
      <n-el class="flex gap-2">
        <n-el
          v-for="item in student.score_list"
          :key="item.id"
          class="px-2 text-xs b rd-2"
          :style="{
            borderColor: `rgb(from ${item.color} r g b / 50%)`,
            color: item.color
          }"
          size="small"
        >
          {{ item.score }}
        </n-el>
      </n-el>
    </template>
  </n-thing>
</template>
