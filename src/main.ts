import { createApp } from "vue"
import App from "./App.vue"
import PrimeVue from 'primevue/config'
import "primevue/resources/themes/lara-light-indigo/theme.css"
import "primevue/resources/primevue.min.css"
import "./styles.css"

const app = createApp(App)
app.use(PrimeVue)
app.mount("#app")
