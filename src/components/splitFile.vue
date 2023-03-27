<template>
  <div class="wrap-grid pad">
    <MyDialog :show="isLoading">
      <div class="center-content">
      <h1 style="text-align: center; margin-bottom: 20px;">Файл разбивается, пожалуйста подождите. <br />(►__◄)</h1>
      <progressBar :progress="precent" ></progressBar>
    </div>
    </MyDialog>
    <dragAndDrop v-model:fileName="fileName" ></dragAndDrop>
    <div class="options-wrap">
      <settingsFile v-model:fileName="fileName" @splitFileRun="splitFileRun"></settingsFile>
    </div>
  </div>
</template>

<script>
import MyDialog from "./MyDialog.vue";
import dragAndDrop from "./dragAndDrop.vue";
import settingsFile from "./settingsFile.vue";
import progressBar from "../components/progressBar.vue";
import eventsMix from "../mixins/eventsMix";
import { invoke } from "@tauri-apps/api";

export default {
  name: "wrap-grid",
  mixins: [eventsMix],
  components: {
    dragAndDrop,
    settingsFile,
    MyDialog,
    progressBar,
  },
  data(){
    return{
        fileName : '',
        isLoading : false,
    }
  },
  methods:{
    showModal(){
      this.isLoading = !this.isLoading;
    },
    async splitFileRun(args) {
        this.showModal();
        await invoke("encode_file", args).finally(() => {
          this.fileName = '';
          this.precent=this.countParts=0;
          this.showModal();
          console.log(args);
          alert(
            `файл был разбит! Части файла и мета-файл найдёте по пути ${args.options.path_for_save}`
          );
          
        });
      } 
    },
};
</script>

<style scoped>
.wrap-grid {
  display: flex;
  justify-content: space-around;
  flex-wrap: wrap;
}



</style>
