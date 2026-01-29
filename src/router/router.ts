import { createMemoryHistory, createRouter } from "vue-router";
import { routes } from "./routes";
import { useAuthStore } from "../stores/auth";

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  const authStore = useAuthStore();

  // Si la ruta requierir sesión y no hay sesión activa
  if (to.meta.requiresAuth && !authStore.isLoggeIn) {
    return next("/login"); // redirige a login
  }

  // Si el usuario está logueado y quiere ir a login, redirige al dashboard
  if (to.path === "/login" && authStore.isLoggeIn) {
    return next("/"); // redirige al dashboard
  }

  // Si no hay condiciones especiales, deja continuar
  next();
});
