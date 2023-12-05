<script lang="tsx" setup>
import { PlusRound as PlusIcon } from "@vicons/material";
import { ScoreType, getScoreTypeList, deleteScoreType } from "@/api/score";
import EditModal from "./components/edit-modal.vue";

const columns = [
  {
    title: "名称",
    key: "name",
  },
  {
    title: "图标",
    key: "icon",
  },
  {
    title: "颜色",
    key: "color",
    render(row: any) {
      return (
        <n-el tag="span" class="px-2 py-1" style={{ backgroundColor: row.color, color: "white" }}>
          {row.color}
        </n-el>
      );
    },
  },
  {
    title: "描述",
    key: "desc",
  },
  {
    title: "操作",
    key: "actions",
    render(row: any) {
      return (
        <n-space>
          <n-button
            type="warning"
            secondary
            size="small"
            onClick={() => open(row.id)}
          >
            编辑
          </n-button>
          <n-popconfirm
            negative-text="取消"
            positive-text="确定"
            onPositiveClick={() => del(row.id)}
            v-slots={{
              trigger: () => (
                <n-button type="error" secondary size="small">
                  删除
                </n-button>
              ),
            }}
          >
            删除了就没有了哦
          </n-popconfirm>
        </n-space>
      );
    },
  },
];
const data = ref<ScoreType[]>([]);

const init = async () => {
  data.value = await getScoreTypeList();
};

const editModalRef = ref<typeof EditModal>();
const open = (id?: string) => editModalRef.value?.open(id);

const del = async (id: string) => {
  console.log(id);
  await deleteScoreType(id);
  await init();
};

init();
</script>

<template>
  <n-space vertical class="p-3">
    <n-space class="p-3 b rd-3">
      <n-button type="primary" @click="() => open()">
        <template #icon>
          <n-icon><plus-icon /></n-icon>
        </template>
        添加
      </n-button>
    </n-space>

    <n-data-table :columns="columns" :data="data">
      <template #empty>
        <n-empty description="暂无数据" />
      </template>
    </n-data-table>
    <edit-modal ref="editModalRef" @success="init" />
  </n-space>
</template>
