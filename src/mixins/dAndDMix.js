import { listen as tauriListen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
export default{
    data() {
        return {
          active: false,
          iconDrop: false,
          tauriListeners: [],
          fileName: "",
        };
      },
      methods: {
        toggleActive() {
          this.active = !this.active;
        },
    
        closeFile() {
          this.iconDrop = false;
          this.fileName = "";
        },
        dropFile(event) {
          this.toggleActive();
          this.iconDrop = true;
          this.fileName = event.payload[0];
          this.$emit("setFileName", this.fileName)
        },
        async openFile() {
          const openedFile = await open({
            multiple: false,
          });
          this.iconDrop = true;
          this.fileName = openedFile;
          this.$emit("setFileName", this.fileName)
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