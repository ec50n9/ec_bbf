<script lang="ts" setup>
import * as PIXI from "pixi.js";
import { SlidingWindow } from "./sliding-window";

const props = defineProps<{
  stream: MediaStream;
}>();

// 创建应用
const rootEle = ref<HTMLDivElement>();
let app = new PIXI.Application({
  background: "#ecfdf5",
  resizeTo: window,
  // resolution: window.devicePixelRatio,
  resolution: 1,
  antialias: true,
});
onMounted(() => {
  setup(app, props.stream);

  rootEle.value?.appendChild(app.view as any);
  return () => {
    rootEle.value?.removeChild(app.view as any);
    app.destroy();
  };
});

const decibel2text = (decibel: number) => {
  if (decibel < 50) return "就这？啊？";
  else if (decibel < 80) return "汗流浃背了吧，老弟";
  else if (decibel < 100) return "有点东西，但是不多";
  else if (decibel < 120) return "算你厉害";
  else if (decibel < 150) return "我嘞个豆！";
  else return "好好好";
};

// 平均数滑动窗口
const slidingWindow = new SlidingWindow(100); // 设置窗口大小为5

// 最大值
let maxValue = 0;

const setup = (app: PIXI.Application, stream: MediaStream) => {
  // 初始化音频流
  const audioCtx = new window.AudioContext();
  const analyser = audioCtx.createAnalyser();
  const source = audioCtx.createMediaStreamSource(stream);
  source.connect(analyser);

  analyser.fftSize = 256;
  const bufferLength = analyser.frequencyBinCount;
  const dataArray = new Uint8Array(bufferLength);

  // 绘制矩形
  const centerCircle = new PIXI.Graphics();
  centerCircle.beginFill("#d1fae5");
  centerCircle.drawCircle(app.screen.width / 2, app.screen.height / 2, 100);
  app.stage.addChild(centerCircle);

  // 绘制文本
  const centerText = new PIXI.Text("Hello World", {
    align: "center",
    fontFamily: "Arial",
    fontSize: 36,
    fontWeight: "bold",
    fill: "#047857",
    dropShadow: true,
    dropShadowColor: "#a7f3d0",
    dropShadowBlur: 4,
    dropShadowAngle: Math.PI / 6,
    dropShadowDistance: 3,
    lineJoin: "round",
  });
  app.stage.addChild(centerText);

  // 实时分贝
  const currentDecibelText = new PIXI.Text("", { fontSize: 18 });
  currentDecibelText.position.set(20, 20);
  app.stage.addChild(currentDecibelText);

  // 平均分贝
  const averageDecibelText = new PIXI.Text("", { fontSize: 18 });
  averageDecibelText.position.set(20, 20 + currentDecibelText.height + 10);
  app.stage.addChild(averageDecibelText);

  // 最大分贝
  const maxDecibelText = new PIXI.Text("", { fontSize: 18 });
  maxDecibelText.position.set(
    20,
    20 + currentDecibelText.height + 10 + averageDecibelText.height + 10
  );
  app.stage.addChild(maxDecibelText);

  // 更新页面
  let lastUpdateCurrentDecibelTime = 0;
  let lastUpdateCenterTextTime = 0;
  app.ticker.add((delta) => {
    analyser.getByteFrequencyData(dataArray);
    const decibelValue =
      dataArray.reduce((a, b) => a + b, 0) / dataArray.length;

    centerCircle.clear();
    centerCircle.beginFill("#d1fae5");
    centerCircle.drawCircle(
      app.screen.width / 2,
      app.screen.height / 2,
      0 + (decibelValue / 255) * (app.screen.width / 2)
    );

    // 更新实时分贝
    if (
      lastUpdateCurrentDecibelTime > 50 &&
      decibelValue !== +currentDecibelText.text
    ) {
      // 实时值
      currentDecibelText.text = `实时：${decibelValue.toFixed(2)}`;

      // 更新平均分贝
      slidingWindow.addNumber(decibelValue);
      averageDecibelText.text = `平均：${slidingWindow
        .getAverage()
        .toFixed(2)}`;

      // 最大值
      if (decibelValue > maxValue) maxValue = decibelValue;
      maxDecibelText.text = `最大：${maxValue.toFixed(2)}`;

      lastUpdateCurrentDecibelTime = 0;
    } else {
      lastUpdateCurrentDecibelTime += delta;
    }

    // 更新中间文字
    if (
      lastUpdateCenterTextTime > 50 &&
      decibel2text(decibelValue) !== centerText.text
    ) {
      centerText.text = decibel2text(decibelValue);
      lastUpdateCenterTextTime = 0;
    } else {
      lastUpdateCenterTextTime += delta;
    }
    const fontSize = (decibelValue / 255) * 100;
    centerText.style.fontSize = `${fontSize}px`;
    centerText.position.set(
      (app.screen.width - centerText.width) / 2,
      (app.screen.height - centerText.height) / 2
    );
  });
};
</script>

<template>
  <div ref="rootEle"></div>
</template>
