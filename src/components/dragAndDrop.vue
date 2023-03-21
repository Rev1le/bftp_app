<template>
  <div @click="openFile"
    class="dropzone"
    :class="{ 'active-dropzone': active }"
  >
    <font-awesome-icon class="upload-icon" icon="fa-solid fa-upload" />

    <span>Перенесите файл</span>
    <span>ИЛИ</span>

    <button>Выберете файл</button>
    
  </div>
  <!-- @dragenter.prevent="toggleActive"
    @dragleave.prevent="toggleActive"
    @dragover.prevent
    @drop.prevent="toggleActive" -->
</template>

<script>
import { listen as tauriListen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
export default {
  data() {
    return {
      active: false,
      tauriListeners: [],
    };
  },
  methods: {
    toggleActive() {
      this.active = !this.active;
    },
    selectFile() {
      this.$emit("selectFile", event);
    },
    romaEvent(event) {
      this.toggleActive();
      console.log(event);
      
    },
    async openFile() {
      const openedFile =  await open({
        multiple: false,
        });
      console.log(openedFile);
    },
    // async unlisten() {
    //   await appWindow.onFileDropEvent((event) => {
    //     if (event.payload.type === "hover") {
    //       console.log("User hovering", event.payload.paths);
    //     } else if (event.payload.type === "drop") {
    //       console.log("User dropped", event.payload.paths);
    //     } else {
    //       console.log("File drop cancelled");
    //     }
    //   });
    // },
  },
  async mounted() {
    this.tauriListeners.push(
      await tauriListen("tauri://file-drop", this.romaEvent)
    );
    this.tauriListeners.push(
      await tauriListen("tauri://file-drop-hover", this.toggleActive)
    );
    this.tauriListeners.push(
      await tauriListen("tauri://file-drop-cancelled", this.toggleActive)
    );
  },
};
</script>

<style scoped>
.dropzone {
  /* position: relative; */
  cursor: pointer;
  width: 100%;
  height: 100%;
  max-height: 500px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  row-gap: 16px;
  border: 3px dashed #5989e9;
  border-radius: 10px;
  transition: all ease 0.3s;
}
.upload-icon {
  font-size: 80px;
  transition: all ease 0.3s;
  position: absolute;
  /* top:50%; */
  opacity: 0;
}

/* input {
  display: none;
} */
span {
  transition: all ease 0.3s;
  font-size: 1.6rem;
}
.active-dropzone {
  border: 3px dashed white;
  background-color: #5989e9;
}
.active-dropzone span,
.active-dropzone button {
  opacity: 0;
  background-color: #5989e9;
}

.active-dropzone .upload-icon {
  opacity: 1;
  background-color: #5989e9;
  color: white;
}
</style>
