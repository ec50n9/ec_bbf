import { RouteRecordRaw, createRouter, createWebHashHistory } from "vue-router";

const routes: RouteRecordRaw[] = [
  { path: "/", component: () => import("@/views/home/index.vue") },
  {
    path: "/group-manager",
    component: () => import("@/views/group-manager/index.vue"),
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
