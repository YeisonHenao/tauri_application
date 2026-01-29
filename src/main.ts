import { createApp } from "vue";
import { createPinia } from "pinia";
import { createPlugin as createTauriPiniaPlugin } from "@tauri-store/pinia";
import App from "./App.vue";
import { router } from "./router/router";
import "./styles/global/global.css";

const app = createApp(App);
const pinia = createPinia();
pinia.use(createTauriPiniaPlugin());

app.use(pinia);
app.use(router);
app.mount("#app");
