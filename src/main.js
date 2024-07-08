import { createApp } from "vue";
import App from "./App.vue";
import router from './router'

// Import Framework7 Bundle
import Framework7 from 'framework7/bundle';

// Import Framework7-Vue with helper to register all components
import Framework7Vue, { registerComponents } from 'framework7-vue/bundle';

// Init plugin and register all components
Framework7.use(Framework7Vue);

const vue = createApp(App);

// register all Framework7 Vue components by passing Vue app instance there
registerComponents(vue);

vue.use(router);
vue.mount("#app");
