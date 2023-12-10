import { LogicalSize } from "@tauri-apps/api/window";
import { defineStore } from "pinia";

export const useAppStore = defineStore("app", {
  state: () => ({
    /** 窗口置顶 */
    isAlwaysOnTop: false,
    /** 迷你窗口模式 */
    miniWindowMode: false,
  }),
  actions: {
    toggleAlwaysOnTop() {
      this.isAlwaysOnTop = !this.isAlwaysOnTop;
      // @ts-ignore
      window.__TAURI__.window.getCurrent().setAlwaysOnTop(this.isAlwaysOnTop);
    },
    toggleMiniWindowMode() {
      this.miniWindowMode = !this.miniWindowMode;
      if (this.miniWindowMode) {
        // @ts-ignore
        window.__TAURI__.window.getCurrent().setSize(new LogicalSize(550, 215));
      }else{
        // @ts-ignore
        window.__TAURI__.window.getCurrent().setSize(new LogicalSize(800, 600));
      }
    },
  },
});
