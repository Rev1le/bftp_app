<template>
  <div class="pad join" style="">
    <MyDialog v-model:show="isLoading">
      <h1 style="text-align: center">
        Файл собирается, пожалуйста подожтите. <br />(►__◄)
      </h1>
    </MyDialog>
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
import ButtonsMix from "../mixins/ButtonsMix";
import MyDialog from "../components/MyDialog.vue";
import { invoke } from "@tauri-apps/api";
export default {
  mixins: [ButtonsMix],
  components: {
    dragAndDrop,
    twoButtons,
    MyDialog,
  },
  data() {
    return {
      fileName: "",
      isLoading: false,
    };
  },
  methods: {
    async joinFileByMeta() {
      if (this.fileName != "" && this.getNewDirectory) {
        this.isLoading = true;
        await invoke("decode_file", {
          filePath: this.fileName,
          pathForSave: this.getNewDirectory,
        }).then(() => {
          this.isLoading = false;
          alert(
            `Процесс завершён!\nВы найдёте файл по пути: ${this.newDirectory}`
          );
          this.newDirectory = "";
          this.fileName = "";
        });
      } else {
        alert("ВЫБЕРИТЕ META ФАЙЛ или путь сохранения собранного файла!");
      }
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
