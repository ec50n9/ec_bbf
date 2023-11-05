<script lang="ts" setup>
import { ArchiveOutline as ArchiveIcon } from "@vicons/ionicons5";
import { UploadCustomRequestOptions } from "naive-ui";

const visible = ref(false);

/** 打开弹窗 */
const open = () => {
  visible.value = true;
};
defineExpose({ open });

/** 解析txt文件 */
const convertTxtFile = (file: File) =>
  new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (event) => {
      let contents = event.target?.result as string;
      console.log(contents);
      resolve(contents);
    };
    reader.onerror = () => {
      reject("文件解析出错");
    };

    reader.readAsText(file);
  });

/** 解析json文件 */
const convertJsonFile = (file: File) =>
  new Promise((resolve, reject) => {
    resolve("");
  });

/** 解析xlsx文件 */
const convertXlsxFile = (file: File) =>
  new Promise((resolve, reject) => {
    resolve("");
  });

/** 处理上传文件 */
const customUploadRequest = ({
  file,
  onFinish,
  onError,
}: UploadCustomRequestOptions) => {
  const targetFile = file.file;
  if (!targetFile) return;

  let func = {
    // 文本读取
    "text/plain": convertTxtFile,
    // json导入
    "application/json": convertJsonFile,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet":
      convertXlsxFile,
  }[targetFile.type];
  if (!func) {
    console.log("未知类型");
    onError();
    return;
  }

  func(targetFile);
  onFinish();
};
</script>

<template>
  <!-- 上传数据 -->
  <n-modal
    v-model:show="visible"
    class="w-4/5 max-w-160"
    preset="card"
    title="通过文件导入"
    size="medium"
    :bordered="false"
    :segmented="{
      content: false,
      footer: false,
    }"
  >
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
  </n-modal>
</template>
