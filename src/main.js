import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import { library } from '@fortawesome/fontawesome-svg-core'

import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faUserSecret, faUpload, faXmark, faWindowMaximize, faRectangleXmark, faWindowMinimize } from '@fortawesome/free-solid-svg-icons'
import { faFile} from '@fortawesome/free-regular-svg-icons'


library.add(faUserSecret, faUpload, faFile, faXmark, faWindowMaximize, faRectangleXmark, faWindowMinimize) 
import './assets/main.css'

createApp(App).component('font-awesome-icon', FontAwesomeIcon).use(router).mount('#app')
