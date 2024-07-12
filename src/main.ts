import "virtual:uno.css";
import "primeicons/primeicons.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";

import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';
import Ripple from "primevue/ripple";
import Toast from "primevue/toast";
import ToastService from "primevue/toastservice";

const app = createApp(App)
const pinia = createPinia()

app.use(PrimeVue, {
    ripple: true,
    theme: {
        preset: Aura,
        options: {
            prefix: 'p',
            darkModeSelector: '.dark',
            cssLayer: false
        }
    }
});
app.use(ToastService);
app.directive("ripple", Ripple);
app.component("Toast", Toast);

app.use(pinia);
app.use(router);

app.mount("#app");
