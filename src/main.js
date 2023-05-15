import { createApp } from "vue";
import { createPinia } from 'pinia'
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/antd.css';
import "./styles.css";
import App from "./App.vue";

const pinia = createPinia()
const app = createApp(App)
app.use(pinia)
app.use(Antd)
app.mount('#app')
