import { createApp } from "vue";
import App from "./App.vue";

import { Button, Input, Notify } from "@nutui/nutui";
import "@nutui/nutui/dist/style.css";

const vue = createApp(App);
vue.use(Button);
vue.use(Input);
vue.use(Notify);
vue.mount("#app");
