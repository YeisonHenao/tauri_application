import { defineStore } from "pinia";
import { AuthUser } from "../types/auth.type";

export const useAuthStore = defineStore("auth", {
  // Estado del AuthStore
  state: () => ({
    user: null as AuthUser | null,
    isLoggeIn: false,
  }),

  // Acciones disponibles
  actions: {
    loginUser(userData: AuthUser) {
      this.user = userData;
      this.isLoggeIn = true;
    },

    logOutUser() {
      this.user = null;
      this.isLoggeIn = false;
    },
  },

  tauri: {
    saveOnChange: true, // Guardar cuando cambie el valor
    saveStrategy: "debounce",
    saveInterval: 500, // Guarda cada 500ms tras cambios
  },
});
