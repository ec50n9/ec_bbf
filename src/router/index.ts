import { RouteRecordRaw, createRouter, createWebHashHistory } from "vue-router";

const routes: RouteRecordRaw[] = [
  { path: "/", component: () => import("@/views/home/index.vue") },
  { path: "/pick-name", component: () => import("@/views/pick-name.vue") },
  {
    path: "/group-manager",
    component: () => import("@/views/group-manager.vue"),
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
