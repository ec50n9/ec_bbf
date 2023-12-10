<script lang="tsx" setup>
import { PlusRound as PlusIcon } from "@vicons/material";
import { getScoreTypeList, deleteScoreType } from "@/api/modules/score";
import EditModal from "./components/edit-modal.vue";
import { useRequest } from "alova";

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
        <n-el
          tag="span"
          class="px-2 py-1"
          style={{ backgroundColor: row.color, color: "white" }}
        >
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
      const show = ref(false);
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
            v-model:show={show.value}
            negative-text="取消"
            positive-text="确定"
            v-slots={{
              trigger: () => (
                <n-button type="error" secondary size="small">
                  删除
                </n-button>
              ),
              action: () => (
                <n-space>
                  <n-button
                    size="small"
                    onClick={() => {
                      show.value = false;
                    }}
                  >
                    取消
                  </n-button>
                  <n-button
                    type="error"
                    size="small"
                    onClick={() => del(row.id)}
                    loading={delScoreTypeLoading.value}
                  >
                    确定
                  </n-button>
                </n-space>
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

const {
  data: scoreTypeList,
  loading: scoreTypeListLoading,
  send: fetchScoreTypeList,
} = useRequest(getScoreTypeList);

const { send: delScoreType, loading: delScoreTypeLoading } = useRequest(
  deleteScoreType,
  {
    immediate: false,
  }
);

const editModalRef = ref<typeof EditModal>();
const open = (id?: string) => editModalRef.value?.open(id);

const currId = ref<string>("");
const del = async (id: string) => {
  currId.value = id;
  await delScoreType(id);
  await fetchScoreTypeList();
};
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

    <n-data-table
      :columns="columns"
      :data="scoreTypeList"
      :loading="scoreTypeListLoading"
    >
      <template #empty>
        <n-empty description="暂无数据" />
      </template>
    </n-data-table>
    <edit-modal ref="editModalRef" @success="fetchScoreTypeList" />
  </n-space>
</template>
