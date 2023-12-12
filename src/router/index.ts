import { RouteRecordRaw, createRouter, createWebHistory } from "vue-router";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: () => import("@/views/home/index.vue"),
    meta: { title: "easy 八宝箱", keepAlive: true },
  },
  {
    path: "/score-type-manager",
    component: () => import("@/views/score-type-manager/index.vue"),
    meta: { title: "分数类型管理" },
  },
  {
    path: "/group-manager",
    component: () => import("@/views/group-manager/index.vue"),
    meta: { title: "分组管理" },
  },
  {
    path: "/student-manager",
    component: () => import("@/views/student-manager/index.vue"),
    meta: { title: "学生管理" },
  },
  {
    path: "/timer",
    component: () => import("@/views/timer/index.vue"),
    meta: { title: "定时器" },
  },
  {
    path: "/decibel-meter",
    component: () => import("@/views/decibel-meter/index.vue"),
    meta: { title: "分贝仪" },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
