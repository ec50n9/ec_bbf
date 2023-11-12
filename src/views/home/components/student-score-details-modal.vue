<script lang="tsx" setup>
import { Student } from "@/api/student";
import {
  Score,
  ScoreType,
  getScoreListByStudentId,
  getScoreTypeList,
  addScore,
} from "@/api/score";
import { getStudentById } from "@/api/student";
import {
  PlusRound as PlusIcon,
  MinusRound as MinusIcon,
} from "@vicons/material";
import { changeColor } from "seemly";
import * as echarts from "echarts";
import { useThemeVars } from "naive-ui";

const themeVars = useThemeVars();

const studentId = ref<Student["id"]>();
const studentInfo = ref<Student>();
const scoreTypeList = ref<ScoreType[]>([]);
const scoreMap = ref<Map<Score["score_type_id"], Score["score"]>>(new Map());

// 刷新数据
const refreshData = async () => {
  scoreTypeList.value = await getScoreTypeList();
  if (!studentId.value) return;
  const scoreList = await getScoreListByStudentId(studentId.value);
  scoreMap.value = scoreList.reduce((map, curr) => {
    map.set(curr.score_type_id, curr.score);
    return map;
  }, new Map());
};

// 弹窗相关
const visible = ref(false);
const open = async (id: Student["id"]) => {
  visible.value = true;
  studentId.value = id;
  studentInfo.value = await getStudentById(id);
  await refreshData();
};
defineExpose({ open });

/**
 * 计算学生对应分数类型的百分比
 * @param scoreType 分数类型
 */
const getScorePercentage = (scoreType: ScoreType) => {
  const score = scoreMap.value.get(scoreType.id);
  if (!score) return 0;
  return Math.round((score / scoreType.max) * 100);
};

/** 综合评分 */
const totalScore = computed(() => {
  const percentage = Math.round(
    scoreTypeList.value.reduce(
      (total, curr) => total + getScorePercentage(curr),
      0
    ) / scoreTypeList.value.length
  );

  let text = "一般般啊小老弟";
  let icon = "i-fluent-emoji:call-me-hand";
  if (percentage === 100) {
    text = "我勒个豆！这谁还分得清你和爱因斯坦啊";
    icon = "i-fluent-emoji:index-pointing-at-the-viewer";
  } else if (percentage >= 90) {
    text = "好好好，算你厉害!";
    icon = "i-fluent-emoji:sports-medal";
  } else if (percentage >= 80) {
    text = "有点东西!";
    icon = "i-fluent-emoji:clapping-hands";
  } else if (percentage >= 50) {
    text = "害行嗷, 再加把劲";
    icon = "i-fluent-emoji:flexed-biceps";
  } else if (percentage >= 40) {
    text = "中规中矩，敷衍你一下吧";
    icon = "i-fluent-emoji:thumbs-up";
  } else if (percentage >= 30) {
    text = "有两把刷子, 但好像只有两把";
    icon = "i-fluent-emoji:person-tipping-hand";
  } else if (percentage >= 20) {
    text = "嘿嘿，好起来了";
    icon = "i-fluent-emoji:cat-with-wry-smile";
  } else if (percentage >= 10) {
    text = "是有点进步空间的";
    icon = "i-fluent-emoji:confetti-ball";
  } else if (percentage > 0) {
    text = "总比没有好";
    icon = "i-fluent-emoji:yawning-face";
  } else {
    text = "汗流浃背了吧老弟";
    icon = "i-fluent-emoji:face-with-hand-over-mouth";
  }

  return {
    percentage,
    text,
    icon,
  };
});

const handlePlus = async (scoreType: ScoreType, plusValue: number) => {
  await addScore({
    student_id: studentId.value!,
    score_type_id: scoreType.id,
    score: plusValue,
  });
  await refreshData();
};

/**
 * 获取虚拟数据
 * @param year 年份
 */
function getVirtualData(year: string) {
  const date = +echarts.time.parse(year + "-09-01");
  const end = +echarts.time.parse(+year + 1 + "-01-07");
  console.log(year + 1, end);
  const dayTime = 3600 * 24 * 1000;
  const data: [string, number][] = [];
  for (let time = date; time <= end; time += dayTime) {
    data.push([
      echarts.time.format(time, "{yyyy}-{MM}-{dd}", false),
      Math.floor(Math.random() * 5),
    ]);
  }
  return data;
}

watch(visible, (value) => {
  // 暂时隐藏
  return;

  if (!value) return;

  nextTick(() => {
    // @ts-ignore
    const myChart = echarts.init(document.querySelector("#myChartDiv"));

    const option = {
      title: {
        top: 0,
        left: "center",
        text: "每日加分情况",
      },
      tooltip: {
        formatter: (params: any) => {
          return params.value[0] + " " + params.value[1];
        },
      },
      series: {
        type: "heatmap",
        coordinateSystem: "calendar",
        data: getVirtualData("2023"),
      },
      visualMap: {
        top: 35,
        left: "center",
        show: true,
        min: 0,
        max: 5,
        inRange: {
          color: [
            "#ebedf0",
            "#c6e48b",
            "#7bc96f",
            "#239a3b",
            "#196127",
            "#196127",
          ],
        },
        type: "piecewise",
        orient: "horizontal",
      },
      calendar: {
        top: 90,
        left: "center",
        range: ["2023-09-01", "2024-01-07"],
        splitLine: {
          show: false,
        },
        dayLabel: {
          firstDay: 1,
          nameMap: "ZH",
        },
        monthLabel: {
          nameMap: "ZH",
        },
        yearLabel: {
          show: false,
        },
        silent: {
          show: true,
        },
        itemStyle: {
          color: "#ccc",
          borderWidth: 3,
          borderColor: "#fff",
          borderRadius: 5,
        },
        // cellSize: ["auto", 13],
        cellSize: [17],
      },
      dataZoom: [
        {
          type: "slider",
          show: true, //flase直接隐藏图形
          xAxisIndex: [0],
          left: "9%", //滚动条靠左侧的百分比
          bottom: -5,
          start: 0, //滚动条的起始位置
          end: 50, //滚动条的截止位置（按比例分割你的柱状图x轴长度）
        },
      ],
    };
    myChart.setOption(option);
  });
});
</script>

<template>
  <n-modal
    v-model:show="visible"
    class="w-4/5 max-w-120"
    preset="card"
    :title="studentInfo?.name"
    size="medium"
    :bordered="false"
    :segmented="{
      content: false,
      footer: false,
    }"
  >
    <!-- 每日加分情况 -->
    <!-- <div id="myChartDiv" class="w-full h-58"></div> -->

    <n-space vertical>
      <!-- 统计面板 -->
      <n-el class="pb-3 flex justify-center items-center gap-3">
        <n-progress
          class="shrink-0"
          type="dashboard"
          :stroke-width="10"
          gap-position="bottom"
          :percentage="totalScore.percentage"
        />
        <n-el class="ml-2">
          <n-el class="mb-1 text-base c-gray-4">我的评价是</n-el>
          <n-el class="text-2xl c-yellow-5">{{ totalScore.text }}</n-el>
        </n-el>
        <n-el class="shrink-0 text-7xl" :class="totalScore.icon" />
      </n-el>

      <!-- 每个分数 -->
      <n-el
        v-for="item in scoreTypeList"
        :key="item.id"
        class="flex items-center gap-3"
      >
        <!-- 分数类型 -->
        <n-el class="shrink-0">{{ item.name }}</n-el>
        <!-- 分数进度条 -->
        <n-popover trigger="hover">
          <template #trigger>
            <n-progress
              class="grow"
              type="line"
              :percentage="getScorePercentage(item)"
              :indicator-placement="'inside'"
              :rail-color="changeColor(themeVars.infoColor, { alpha: 0.2 })"
            >
              <n-el>{{ scoreMap.get(item.id) || 0 }} 分</n-el>
            </n-progress>
          </template>
          {{ item.desc }}
        </n-popover>

        <n-space class="shrink-0">
          <!-- 扣分 -->
          <n-button
            type="error"
            secondary
            size="small"
            circle
            @click="() => handlePlus(item, -1)"
            :disabled="scoreMap.get(item.id) === 0"
          >
            <template #icon>
              <n-icon><minus-icon /></n-icon>
            </template>
          </n-button>
          <!-- 加分 -->
          <n-button
            type="success"
            secondary
            size="small"
            circle
            @click="() => handlePlus(item, 1)"
            :disabled="scoreMap.get(item.id) === item.max"
          >
            <template #icon>
              <n-icon><plus-icon /></n-icon>
            </template>
          </n-button>
        </n-space>
      </n-el>
    </n-space>
  </n-modal>
</template>
