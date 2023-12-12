<script lang="ts" setup>
import { useElementSize } from "@vueuse/core";

const props = defineProps<{
  stream?: MediaStream;
}>();

const canvasWrapper = ref<HTMLDivElement | null>(null);
const canvasWrapperSize = useElementSize(canvasWrapper);

const canvas = ref<HTMLCanvasElement | null>(null);

watch(
  () => props.stream,
  (stream) => {
    if (stream) {
      const audioCtx = new window.AudioContext();
      const analyser = audioCtx.createAnalyser();
      const source = audioCtx.createMediaStreamSource(stream);
      source.connect(analyser);
      // 连接声音
      // analyser.connect(audioCtx.destination);

      analyser.fftSize = 256;
      const bufferLength = analyser.frequencyBinCount;
      const dataArray = new Uint8Array(bufferLength);

      const canvasCtx = canvas.value?.getContext("2d");
      if (!canvas.value || !canvasCtx) return;

      const draw = () => {
        requestAnimationFrame(draw);

        analyser.getByteFrequencyData(dataArray);
        const maxValue = Math.max(...dataArray);

        // 清空画布
        canvasCtx.fillStyle = "#ecfdf5";
        canvasCtx.fillRect(0, 0, canvas.value!.width, canvas.value!.height);

        const barWidth = (canvas.value!.width / bufferLength) * 5;
        for (let i = 0, x = 0, barHeight = 0; i < bufferLength; i++) {
          barHeight = dataArray[i] / 2;

          // 绘制柱状
          canvasCtx.fillStyle = "#047857";
          canvasCtx.fillRect(
            x,
            canvas.value!.height - barHeight / 2,
            barWidth,
            barHeight
          );

          // 绘制文字
          canvasCtx.font = "12px Arial";
          canvasCtx.fillText(
            `${dataArray[i]}`,
            x,
            canvas.value!.height - barHeight
          );

          x += barWidth + 3;
        }

        // 在中间绘制圆
        canvasCtx.beginPath();
        canvasCtx.fillStyle = "#10b981";
        canvasCtx.arc(
          canvas.value!.width / 2,
          canvas.value!.height / 3,
          70 - maxValue / 10,
          0,
          2 * Math.PI
        );
        canvasCtx.fill();
        canvasCtx.closePath();

        // 绘制外圈
        canvasCtx.beginPath();
        canvasCtx.strokeStyle = "#6ee7b7";
        canvasCtx.lineWidth = maxValue / 8;
        canvasCtx.arc(
          canvas.value!.width / 2,
          canvas.value!.height / 3,
          80,
          0,
          2 * Math.PI
        );
        canvasCtx.stroke();
        canvasCtx.closePath();

        // 在圆上绘制文字
        canvasCtx.beginPath();
        canvasCtx.fillStyle = `rgba(255, 0, 0, ${maxValue / 300})`;
        canvasCtx.font = `${maxValue / 4}px Arial`;
        canvasCtx.textAlign = "center";
        canvasCtx.textBaseline = "middle";

        let content = "就这？";
        if (maxValue > 100) {
          content = "汗流浃背了吧";
        }
        if (maxValue > 150) {
          content = "有点东西";
        }
        if (maxValue > 180) {
          content = "还行吧";
        }
        if (maxValue > 230) {
          content = "算你厉害";
        }
        if (maxValue > 240) {
          content = "太大声啦！";
        }

        // let suffix = "";
        // const subValue = maxValue - 200;
        // if (subValue > 0) {
        //   suffix = "!".repeat(subValue / 10);
        // }
        canvasCtx.fillText(
          content,
          canvas.value!.width / 2,
          canvas.value!.height / 3
        );
        canvasCtx.closePath();
      };
      draw();
    }
  }
);
</script>

<template>
  <n-el ref="canvasWrapper" class="flex-grow">
    <canvas
      ref="canvas"
      class="w-full h-full"
      :width="canvasWrapperSize.width.value"
      :height="canvasWrapperSize.height.value"
    />
  </n-el>
</template>
