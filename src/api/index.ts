import { createAlova } from "alova";
import TauriMockAdapter from "./adapters/tauri-adapter";
import VueHook from "alova/vue";

export const request = createAlova({
  baseURL: '',
  statesHook: VueHook,
  requestAdapter: TauriMockAdapter,
  responded: (response) => response.json(),
});
