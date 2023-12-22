<script lang="ts" setup>
import * as PIXI from "pixi.js";
import { Button } from "@pixi/ui";

const props = defineProps<{
  stream: MediaStream;
}>();

// 创建应用
const rootEle = ref<HTMLDivElement>();
let app = new PIXI.Application({
  background: "#ecfdf5",
  resizeTo: window,
  resolution: window.devicePixelRatio || 1,
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
  if (decibel < 100) return "就这？啊？";
  else if (decibel < 150) return "汗流浃背了吧，老弟";
  else if (decibel < 200) return "有点东西，但是不多";
  else if (decibel < 230) return "算你厉害";
  else if (decibel < 250) return "我嘞个豆！";
  else return "好好好";
};

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

  // 按钮
  const container = new PIXI.Container();
  const button = new Button(
    new PIXI.Graphics().beginFill(0xffffff).drawRoundedRect(0, 0, 100, 50, 15)
  );
  button.onPress.connect(() => {
    console.log("hello");
  });
  container.addChild(button.view);
  app.stage.addChild(container);

  app.ticker.add((_delta) => {
    analyser.getByteFrequencyData(dataArray);
    const decibelValue =
      dataArray.reduce((a, b) => a + b, 0) / dataArray.length;

    centerCircle.clear();
    centerCircle.beginFill("#d1fae5");
    centerCircle.drawCircle(
      app.screen.width / 2,
      app.screen.height / 2,
      100 + (decibelValue / 255) * 700
    );

    centerText.text = decibelValue.toFixed(0);
    // basicText.text = decibel2text(decibelValue);
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
