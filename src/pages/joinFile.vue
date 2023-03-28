<template>
  <div class="pad join" style="">
    <!-- <MyDialog :show="isLoading">
      <div class="center-content">
      <h1 style="text-align: center; margin-bottom: 20px;">Файл разбивается, пожалуйста подождите.</h1>
      <img style="width: 80%;" src="../assets/cat.gif" alt="">
     
      <progressBar :progress="precent" ></progressBar>
    </div>
    </MyDialog> -->
    <loadWindow :precent="precent" :show="isLoading"></loadWindow>
    <div class="conten-center">
      <dragAndDrop v-model:fileName="fileName" :join="true"></dragAndDrop>
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
import ButtonsMix from "../mixins/ButtonsMix";
import loadWindow from "../components/loadWindow.vue";
import progressBar from "../components/progressBar.vue";
import eventsMix from "../mixins/eventsMix";
import { invoke } from "@tauri-apps/api";
import { listen as tauriListen } from "@tauri-apps/api/event";
export default {
  mixins: [ButtonsMix, eventsMix ],
  components: {
    dragAndDrop,
    twoButtons,
    loadWindow,
    progressBar,
  },
  data() {
    return {
      fileName: "",
      isLoading: false,
    };
  },
  methods: {
    showModal(){
      this.isLoading = !this.isLoading;
    },
    async joinFileByMeta() {
      if (this.fileName != "" && this.getNewDirectory) {
        this.isLoading = true;
        let args = {
        filePath: this.fileName,
        options: {
          path_for_save: this.getNewDirectory,
          count_parts: null,
          part_size: null,
          compressed: null,
        },
      }
        await invoke("decode_file", args).then(() => {
          this.isLoading = false;
          alert(
            `Процесс завершён!\nВы найдёте файл по пути: ${this.newDirectory}`
          );
          this.precent=this.countParts=0;
          this.newDirectory = this.fileName = "";
        });
      } else {
        alert("ВЫБЕРИТЕ META ФАЙЛ или путь сохранения собранного файла!");
      }
    },
  },
  async mounted(){
        await tauriListen("decode://count_parts", this.setCountParts);
        await tauriListen("decode://progress", this.setPrecent);
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
