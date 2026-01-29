import Login from "../views/Login/Login.vue";
import Dashboard from "../views/Dashboard/Dashboard.vue";

export const routes = [
  {
    path: "/login",
    name: "Iniciar sesión",
    component: Login,
    meta: {
      requiresAuth: false,
    },
  },
  {
    path: "/",
    name: "Dashboard",
    component: Dashboard,
    meta: {
      requiresAuth: true,
    },
  },
];
