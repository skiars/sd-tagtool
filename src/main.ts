import {createApp} from 'vue'
import {createRouter, createWebHistory} from 'vue-router'
import App from './App.vue'
import Home from './components/Home.vue'
import Settings from './components/Settings.vue'
import PrimeVue from 'primevue/config'
import ToastService from 'primevue/toastservice'
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
app.use(ToastService)
app.mount("#app")

// disable content menu
document.addEventListener('contextmenu', event => event.preventDefault());

// prevent default key events
const modifyKeys = new Set(["KeyR", "KeyP", "KeyF"])
document.addEventListener('keydown', event => {
  const modifier = event.ctrlKey || event.metaKey
  if (modifier && modifyKeys.has(event.code) || event.code == 'F5')
    event.preventDefault()
})
