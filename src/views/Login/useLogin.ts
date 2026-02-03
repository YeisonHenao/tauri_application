import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import { useAuthStore } from "../../stores/auth";
import { ref } from "vue";
import type { AuthUser } from "../../types/auth.type";

export const useLogin = () => {
  const router = useRouter();
  const authStore = useAuthStore();

  const user = ref("");
  const password = ref("");
  const responseMessage = ref("");
  const loading = ref(false);

  const makeLogin = async () => {
    loading.value = true;
    responseMessage.value = "";

    try {
      const authUser = await login(user.value, password.value);
      authStore.loginUser(authUser);
      router.push("/");
    } catch (error: any) {
      responseMessage.value = error?.message ?? "Credenciales incorrectas";
    } finally {
      loading.value = false;
    }
  };

  const login = async (nombre: string, password: string): Promise<AuthUser> => {
    const response = await invoke<AuthUser>("login", {
      nombre,
      password,
    });
    return {
      ...response,
      fecha_creacion: new Date(response.fecha_creacion),
      ultimo_login: new Date(response.ultimo_login),
    };
  };

  return {
    user,
    password,
    responseMessage,
    loading,
    makeLogin,
  };
};
