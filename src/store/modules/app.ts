import { defineStore } from "pinia";

export const useAppStore = defineStore("app", {
  state: () => ({
    /** 窗口置顶 */
    isAlwaysOnTop: false,
  }),
  actions: {
    toggleAlwaysOnTop() {
      this.isAlwaysOnTop = !this.isAlwaysOnTop;
      // @ts-ignore
      window.__TAURI__.window.getCurrent().setAlwaysOnTop(this.isAlwaysOnTop);
    },
  },
});
