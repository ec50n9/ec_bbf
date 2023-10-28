import { createApp } from "vue";
import App from "./App.vue";

import "@unocss/reset/tailwind-compat.css";
import "virtual:uno.css";
import "./styles.css";
// 通用字体
import "vfonts/Lato.css";
// 等宽字体
import "vfonts/FiraCode.css";

createApp(App).mount("#app");
