<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ArchiveOutline as ArchiveIcon } from "@vicons/ionicons5";
import { UploadCustomRequestOptions } from "naive-ui";

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

const customUploadRequest = ({
  file,
  onFinish,
  onError,
}: UploadCustomRequestOptions) => {
  const targetFile = file.file;
  if (!targetFile) return;

  let func = {
    // 文本读取
    "text/plain": () => {
      const reader = new FileReader();
      reader.onload = (event) => {
        let contents = event.target?.result as string;
        console.log(contents);
      };
      reader.readAsText(targetFile);
    },
    // json导入
    "application/json": () => {},
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet": () => {
      console.log("excel导入");
    },
  }[targetFile.type];
  if (!func) {
    console.log("未知类型");
    onError();
    return;
  }
  func();
  onFinish();
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
    <n-upload
      multiple
      directory-dnd
      :max="1"
      :custom-request="customUploadRequest"
    >
      <n-upload-dragger>
        <div style="margin-bottom: 12px">
          <n-icon size="48" :depth="3">
            <archive-icon />
          </n-icon>
        </div>
        <n-text style="font-size: 16px">
          点击或者拖动文件到该区域来上传
        </n-text>
        <n-p depth="3" style="margin: 8px 0 0 0">
          请不要上传敏感数据，比如你的银行卡号和密码，信用卡号有效期和安全码
        </n-p>
      </n-upload-dragger>
    </n-upload>
  </div>
</template>
