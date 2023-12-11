<script lang="ts" setup>
import { useDevicesList, useUserMedia, useElementSize } from "@vueuse/core";

const { audioInputs: microphones } = useDevicesList({
  requestPermissions: true,
});
const currentMicrophone = computed(() => microphones.value[0]?.deviceId);

const { stream, enabled } = useUserMedia({
  constraints: {
    // @ts-ignore
    audio: { deviceId: currentMicrophone },
  },
});

const canvasWrapper = ref<HTMLDivElement | null>(null);
const canvasWrapperSize = useElementSize(canvasWrapper);

const canvas = ref<HTMLCanvasElement | null>(null);

watchEffect(() => {
  if (stream.value) {
    const audioCtx = new window.AudioContext();
    const analyser = audioCtx.createAnalyser();
    const source = audioCtx.createMediaStreamSource(stream.value);
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
      canvasCtx.font = `${maxValue / 3}px Arial`;
      canvasCtx.textAlign = "center";
      canvasCtx.textBaseline = "middle";

      let suffix = "";
      const subValue = maxValue - 200;
      if (subValue > 0) {
        suffix = "!".repeat(subValue / 10);
      }
      canvasCtx.fillText(
        `${maxValue}${suffix}`,
        canvas.value!.width / 2,
        canvas.value!.height / 3
      );
      canvasCtx.closePath();
    };
    draw();
  }
});
</script>

<template>
  <n-el class="w-full h-full flex flex-col">
    <n-switch v-model:value="enabled" />
    <n-el ref="canvasWrapper" class="flex-grow">
      <canvas
        ref="canvas"
        class="w-full h-full"
        :width="canvasWrapperSize.width.value"
        :height="canvasWrapperSize.height.value"
      />
    </n-el>
    <n-empty v-if="!microphones.length" description="没有麦克风" />
  </n-el>
</template>
