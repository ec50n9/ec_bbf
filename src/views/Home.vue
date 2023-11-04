<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri";
import { NButton } from "naive-ui";

const handleList = async () => {
  const res = await invoke("get_list", {});
  console.log(res);
};

const handleCreate = async () => {
  const res = await invoke("create", {
    student: {
      id: "1",
      stu_no: "123",
      name: "梁从心",
      is_delete: false,
    },
  });
  console.log(res);
};
</script>

<template>
  <div>
    <router-link to="/">首页</router-link>
    <router-link to="/pick-name">点名</router-link>
    <n-button @click="handleList">查询</n-button>
    <n-button @click="handleCreate">新增</n-button>
  </div>
</template>
