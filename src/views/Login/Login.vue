<template>
  <div class="contenedor">
    <form>
      <div class="form-contenedor floating">
        <input type="text" id="txtUsuario" name="txtUsuario" v-model="user" required />
        <label for="txtUsuario">Usuario</label>
      </div>
      <div class="form-contenedor floating">
        <input type="password" id="txtPassword" name="txtPassword" v-model="password" required />
        <label for="txtPassword">Contraseña</label>
      </div>
      <div class="button-contenedor">
        <button type="button" @click="makeLogin">Ingresar</button>
      </div>
      <div style="margin-top: 3rem; padding-left: 2rem;">
        <span>{{ responseMessage }}</span>
      </div>
    </form>
  </div>
</template>

<style scope>
@import "../../styles/views/Login/Login.css";
</style>

<script setup lang="ts">
import { ref } from "vue";
import { useLogin } from "./useLogin";

const user = ref("");
const password = ref("");
const responseMessage = ref("");

const { login } = useLogin();

const makeLogin = async () => {
  try {
    console.log("vista! : ", user.value, password.value)
    const result = await login(user.value, password.value);
    responseMessage.value = `La respuesta es ${result}`;
    console.log(result);
  } catch (error) {
    console.error(error);
  }
};
</script>
