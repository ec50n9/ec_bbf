export type PickStatus = "idle" | "running" | "paused" | "stop";

export const usePick = (sourceList: Ref<string[]>, timeout: number = 100) => {
  const allList = computed(() => [...sourceList.value]);

  /** 已选择的值 */
  const selectedList = ref(new Set<string>());

  /** 未选择过的值 */
  const unselectedList = computed(() =>
    allList.value.filter((item) => !selectedList.value.has(item))
  );

  /** 当前聚焦的值 */
  const currentFocusValue = ref<string | null>(null);

  /** 定时器 */
  const timer = ref<ReturnType<typeof setInterval> | null>(null);

  const status = computed<PickStatus>(() => {
    if (timer.value !== null)
      // 有定时器，运行中
      return "running";
    else {
      // 没有定时器，并且没有可选值，停止
      if (unselectedList.value.length === 0) return "stop";
      if (selectedList.value.size === 0)
        // 没有定时器，并且没有选中任何值，闲置
        return "idle";
      // 没有定时器，选中了值，暂停
      else return "paused";
    }
  });

  const run = () => {
    timer.value && clearInterval(timer.value);
    timer.value = setInterval(() => {
      // 如果没有未选择过的值，清除定时器
      if (unselectedList.value.length === 0) {
        timer.value && clearInterval(timer.value);
        return;
      }

      // 从未选择过的值中随机选一个
      const index = Math.floor(Math.random() * unselectedList.value.length);
      currentFocusValue.value = unselectedList.value[index];
    }, timeout);
  };

  const pause = () => {
    timer.value && clearInterval(timer.value);
    timer.value = null;
    currentFocusValue.value = null;
  };

  const reset = () => {
    pause();
    selectedList.value.clear();
  };

  const select = () => {
    const value = currentFocusValue.value;
    value && selectedList.value.add(value);
    // 如果没有未选择过的值，清除定时器
    if (unselectedList.value.length === 0) pause();
    return value;
  };

  const unselect = (value: string) => {
    selectedList.value.delete(value);
  }

  return {
    selectedList,
    unselectedList,
    currentFocusValue,
    run,
    pause,
    reset,
    select,
    unselect,
    status,
  };
};
