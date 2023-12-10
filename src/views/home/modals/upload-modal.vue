<script lang="ts" setup>
import * as XLSX from "xlsx";
import { ArchiveOutline as ArchiveIcon } from "@vicons/ionicons5";
import { UploadCustomRequestOptions } from "naive-ui";
import { StudentCreateVO } from "@/api/types/student";

const emit = defineEmits<{
  (e: "upload", studentList: StudentCreateVO[]): void;
}>();

const visible = ref(false);

/** 打开弹窗 */
const open = () => {
  visible.value = true;
};
defineExpose({ open });

/** 以文本方式读取文件内容 */
const readTextFromFile = (file: File) =>
  new Promise<string>((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (event) => resolve(event.target?.result as string);
    reader.onerror = () => reject("文件解析出错");
    reader.readAsText(file);
  });

/** 以缓冲数组方式读取文件内容 */
const readArrayBufferFromFile = (file: File) =>
  new Promise<ArrayBuffer>((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (event) => resolve(event.target?.result as ArrayBuffer);
    reader.onerror = () => reject("文件解析出错");
    reader.readAsArrayBuffer(file);
  });

/** 解析txt文件 */
const convertTxtFile = (file: File): Promise<StudentCreateVO[]> =>
  readTextFromFile(file).then((text) =>
    text
      .trim()
      .split(/\r\n|\n/)
      .map((line) => line.split(/\s+/))
      .filter((item) => item.length >= 2)
      .map(([name, stu_no]) => ({ name, stu_no }))
  );

/** 解析json文件 */
const convertJsonFile = async (file: File): Promise<StudentCreateVO[]> => {
  const text = await readTextFromFile(file);
  try {
    const dataList = JSON.parse(text) as StudentCreateVO[];
    // 检查数据
    if (dataList.length && dataList.every((item) => item.name && item.stu_no))
      return dataList;
    return [];
  } catch (err) {
    return [];
  }
};

/** 解析xlsx文件 */
const convertXlsxFile = async (file: File): Promise<StudentCreateVO[]> => {
  const plainContent = await readArrayBufferFromFile(file);
  const workbook = XLSX.read(plainContent, { type: "buffer" });
  const firstSheetName = workbook.SheetNames[0];
  const dataList = XLSX.utils.sheet_to_json<StudentCreateVO>(
    workbook.Sheets[firstSheetName]
  );
  // 检查数据
  if (dataList.length && dataList.every((item) => item.name && item.stu_no))
    return dataList.map((item) => ({
      name: item.name + "",
      stu_no: item.stu_no + "",
    }));
  return [];
};

/** 处理上传文件 */
const customUploadRequest = async ({
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

  const studentList: StudentCreateVO[] = await func(targetFile);
  onFinish();
  emit("upload", studentList);
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
      accept=".txt,.json,.xlsx"
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
