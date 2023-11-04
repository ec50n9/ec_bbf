<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri";
import { NButton } from "naive-ui";

const handleList = async () => {
  const res = await invoke("get_student_list", {});
  console.log(res);
};

const handleCreate = async () => {
  const res = await invoke("create_student", {
    studentCreateVo: {
      id: "1",
      stu_no: "123",
      name: "梁从心",
      is_delete: false,
    },
  });
  console.log(res);
};

const handleUpdate = async () => {
  const res = await invoke("update_student", {
    studentUpdateVo: {
      id: "1",
      name: "梁钉钉",
    },
  });
  console.log(res);
};

const handleDelete = async () => {
  const res = await invoke("delete_student", {
    id: "1",
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
    <n-button @click="handleUpdate">更新</n-button>
    <n-button @click="handleDelete">删除</n-button>
  </div>
</template>
