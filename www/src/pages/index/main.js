import Vue from "vue";
import App from "./App.vue";
import store from "@/store/gui.js";
import "@/assets/styles/index.css";

Vue.config.productionTip = false;

window.app = new Vue({
  store,
  render: (h) => h(App),
}).$mount("#app");
