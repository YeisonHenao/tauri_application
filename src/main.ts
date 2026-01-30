import { createApp } from "vue";
import { createPinia } from "pinia";
import { isTauri } from "./utils/isTauri";
import App from "./App.vue";
import { router } from "./router/router";
import "./styles/global/global.css";

const app = createApp(App);
const pinia = createPinia();

if (isTauri()) {
  const { createPlugin } = await import("@tauri-store/pinia");
  pinia.use(createPlugin());
}

app.use(pinia);
app.use(router);
app.mount("#app");
