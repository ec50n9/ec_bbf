<script lang="ts" setup>
import { useDevicesList, useUserMedia } from "@vueuse/core";

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

const canvas = ref<HTMLCanvasElement | null>(null);

watchEffect(() => {
  if (stream.value) {
    const audioCtx = new window.AudioContext();
    const analyser = audioCtx.createAnalyser();
    const source = audioCtx.createMediaStreamSource(stream.value);
    source.connect(analyser);
    analyser.connect(audioCtx.destination);

    analyser.fftSize = 256;
    const bufferLength = analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);

    const canvasCtx = canvas.value?.getContext("2d");
    if (!canvas.value || !canvasCtx) return;

    canvasCtx.clearRect(0, 0, canvas.value.width, canvas.value.height);
    const draw = () => {
      requestAnimationFrame(draw);

      analyser.getByteFrequencyData(dataArray);
      canvasCtx.fillStyle = "rgb(0, 0, 0)";
      canvasCtx.fillRect(0, 0, canvas.value!.width, canvas.value!.height);

      const barWidth = (canvas.value!.width / bufferLength) * 2.5;
      for (let i = 0, x = 0, barHeight = 0; i < bufferLength; i++) {
        barHeight = dataArray[i] / 2;

        canvasCtx.fillStyle = "rgb(" + (barHeight + 100) + ",50,50)";
        canvasCtx.fillRect(
          x,
          canvas.value!.height - barHeight / 2,
          barWidth,
          barHeight
        );

        x += barWidth + 1;
      }
    };
    draw();
  }
});
</script>

<template>
  <n-el>
    hello,
    <n-switch v-model:value="enabled" />
    <canvas ref="canvas" width="800" height="200"></canvas>
  </n-el>
</template>
