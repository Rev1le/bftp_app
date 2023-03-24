<template>
  <div class="pad join" style="">
    <div class="conten-center">
      <dragAndDrop v-model:fileName="fileName"></dragAndDrop>
    </div>
    <twoButtons
      btnOne="Выберете директорию"
      btnTwo="Собрать файл!"
      :callbackOne="newDirFile"
      :callbackTwo="joinFileByMeta"
      :getNewDirectory="this.getNewDirectory"
    ></twoButtons>
 
  </div>
</template>

<script>
import dragAndDrop from "../components/dragAndDrop.vue";
import twoButtons from "../components/twoButtons.vue";
import ButtonsMix from '../mixins/ButtonsMix'

import { invoke } from "@tauri-apps/api";
export default {
  mixins:[ButtonsMix],
  components: {
    dragAndDrop,
    twoButtons
  },
  data() {
    return {
      fileName: "",
      isLoading : false,
    };
  },
  methods: {
    async joinFileByMeta() {
      this.isLoading = true;
      await invoke("decode_file", {
        filePath: this.fileName,
        pathForSave: this.getNewDirectory,
      }).then(()=>{
        this.isLoading = false;
        alert("процесс завершён");
      });
    },
    
  },
};
</script>

<style scoped>

.conten-center {
  display: flex;
  align-items: stretch;
  justify-content: center;
  height: 80%;
  width: 100%;
}

.join {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
}

button {
  justify-self: center;
  width: 300px;
}
</style>
