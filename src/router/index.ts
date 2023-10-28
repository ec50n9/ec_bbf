import { RouteRecordRaw, createRouter, createWebHashHistory } from "vue-router";

const routes: RouteRecordRaw[] = [
  { path: "/", component: () => import("@/views/Home.vue") },
  { path: "/pick-name", component: () => import("@/views/PickName.vue") },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
