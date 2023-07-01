import {createApp} from 'vue'
import {createRouter, createWebHistory} from 'vue-router'
import App from './App.vue'
import Home from './components/Home.vue'
import Settings from './components/Settings.vue'
import PrimeVue from 'primevue/config'
import "primevue/resources/themes/lara-light-indigo/theme.css"
import "primevue/resources/primevue.min.css"
import "./styles.css"

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {path: '/', component: Home},
    {path: '/settings', component: Settings}
  ]
})

const app = createApp(App)
app.use(router)
app.use(PrimeVue)
app.mount("#app")

// disable content menu
document.addEventListener('contextmenu', event => event.preventDefault());
