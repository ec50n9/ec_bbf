<script lang="ts" setup>
const props = defineProps<{
  stream?: MediaStream;
}>();

const state = reactive({
  average: 0,
  max: 0,
  current: 0,
});

const sensitivity = ref(0.5);
const sensitivityStep = ref(0.1);

const calculateDecibel = (dataArray: Uint8Array) => {
  const values = dataArray.reduce((acc, curr) => acc + curr, 0);
  const average = values / dataArray.length;
  const max = Math.max(...dataArray);
  return { average, max };
};

const updateDecibelDisplay = (
  analyser: AnalyserNode,
  dataArray: Uint8Array
) => {
  analyser.getByteFrequencyData(dataArray);
  const { average, max } = calculateDecibel(dataArray);
  state.average = average;
  state.max = max;
  state.current = dataArray[0];
  requestAnimationFrame(() => updateDecibelDisplay(analyser, dataArray));
};

const handleStream = (stream: MediaStream) => {
  // 创建音频上下文
  const audioCtx = new window.AudioContext();
  const source = audioCtx.createMediaStreamSource(stream);

  // 创建分析器
  const analyser = audioCtx.createAnalyser();
  analyser.fftSize = 256;
  const bufferLength = analyser.frequencyBinCount;
  const dataArray = new Uint8Array(bufferLength);

  // 连接音频源和分析器
  source.connect(analyser);

  updateDecibelDisplay(analyser, dataArray);
};
</script>

<template>
  <div>
    <p>
      平均分贝: <span>{{ state.average }}</span> dB
    </p>
    <p>
      最高分贝: <span>{{ state.max }}</span> dB
    </p>
    <p>
      实时分贝: <span>{{ state.current }}</span> dB
    </p>
  </div>
</template>
