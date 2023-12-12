<script lang="ts" setup>
import * as PIXI from "pixi.js";

const props = defineProps<{
  stream: MediaStream;
}>();

// 创建应用
const rootWrapperEle = ref<HTMLDivElement>();
const rootEle = ref<HTMLDivElement>();
let app = new PIXI.Application({ background: "#ecfdf5", resizeTo: window });
onMounted(() => {
  setup(app, props.stream);

  rootEle.value?.appendChild(app.view as any);
  return () => {
    rootEle.value?.removeChild(app.view as any);
    app.destroy();
  };
});

const easeInCubic = (x: number) => x * x * x;

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
  const obj = new PIXI.Graphics();
  obj.beginFill(0xff0000);
  obj.drawCircle(0, 0, 100);
  app.stage.addChild(obj);

  // 绘制文本
  const basicText = new PIXI.Text("Hello World", {
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
  app.stage.addChild(basicText);

  let time = 0;

  app.ticker.add((delta) => {
    // time += delta;
    // if (time > 5) time = 0;
    // else return;

    analyser.getByteFrequencyData(dataArray);
    // const decibelValue = Math.max(...dataArray);
    const decibelValue = dataArray.reduce((a, b) => a + b, 0) / dataArray.length;
    // console.log(easeInCubic((avg / 255) * 10));

    basicText.text = decibel2text(decibelValue);
    const fontSize = decibelValue / 50 * 80
    basicText.style.fontSize = `${fontSize}px`;
    basicText.position.set(
      (app.screen.width - basicText.width) / 2,
      (app.screen.height - basicText.height) / 2
    );
  });
};
</script>

<template>
  <div ref="rootWrapperEle" class="w-full h-full bg-red-1">
    <div ref="rootEle"></div>
  </div>
</template>
