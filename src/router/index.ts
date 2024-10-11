import { createMemoryHistory, createRouter } from "vue-router";

const routes = [
  { path: "/", component: () => import("../views/Home.vue") },
  {
    path: "/login",
    component: () => import("../views/Login.vue"),
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
