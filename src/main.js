import { createApp } from 'vue'
import App from './App.vue'
import { library } from '@fortawesome/fontawesome-svg-core'
// import { listen } from '@tauri-apps/api/event'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faUserSecret, faUpload } from '@fortawesome/free-solid-svg-icons'
import { faFile } from '@fortawesome/free-regular-svg-icons'
// import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
// import { event } from '@tauri-apps/api'

// appWindow.emit('event', { message: 'Tauri is awesome!' })

// create a new webview window and emit an event only to that window


// console.log(listen);
/* add icons to the library */

// const fileDrop = event.listen('tauri://file-drop', (e) => {
//     const payload = e.payload;
//     files = getValidPaths(payload)
//     if (files.length > 0) {
//       handleFiles(files)
//     }
//     if (payload.length === 1 && files.length === 1) {
//       handleOneFile(files[0])
//     }
//     files = []
//   })

// console.log(typeof fileDrop, fileDrop);

library.add(faUserSecret, faUpload, faFile) 
import './assets/main.css'

createApp(App).component('font-awesome-icon', FontAwesomeIcon).mount('#app')
