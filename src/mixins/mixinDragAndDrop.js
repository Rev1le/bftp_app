import { listen as tauriListen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
export default{
    props:{
      fileName : "",
      join:false,
    },
    data() {
        return {
          active: false,
          iconDrop: false,
          tauriListeners: [],
        };
      },
      methods: {
        toggleActive() {
          this.active = !this.active;
        },
    
        closeFile() {
          this.iconDrop = false;
          this.$emit("update:fileName", "");
        },
        dropFile(event) {
          this.toggleActive();
          this.iconDrop = true;
          this.$emit("update:fileName", event.payload[0])
        },
        async openFile() {
          let args = {
            multiple: false,
          }
          if(this.join){
            const filters =  [{
              name: 'build_file',
              extensions: ['meta']
            }]
            args.filters = filters;
          }
          const openedFile = await open(args);
          if(openedFile!=null){
            this.iconDrop = true;
            this.$emit("update:fileName", openedFile)
          }
          
        },
      },
      async mounted() {
        this.tauriListeners.push(
          await tauriListen("tauri://file-drop", this.dropFile)
        );
        this.tauriListeners.push(
          await tauriListen("tauri://file-drop-hover", this.toggleActive)
        );
        this.tauriListeners.push(
          await tauriListen("tauri://file-drop-cancelled", this.toggleActive)
        );
      },
}