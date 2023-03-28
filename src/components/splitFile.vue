<template>
  <div class="wrap-grid pad">
    <loadWindow :precent="precent" :show="isLoading"></loadWindow>
    <dragAndDrop v-model:fileName="fileName" ></dragAndDrop>
    <div class="options-wrap">
      <settingsFile v-model:fileName="fileName" @splitFileRun="splitFileRun"></settingsFile>
    </div>
  </div>
</template>

<script>

import dragAndDrop from "./dragAndDrop.vue";
import settingsFile from "./settingsFile.vue";

import eventsMix from "../mixins/eventsMix";
import { invoke } from "@tauri-apps/api";
import { listen as tauriListen } from "@tauri-apps/api/event";
import loadWindow from "./loadWindow.vue";
export default {
  name: "wrap-grid",
  mixins: [eventsMix],
  components: {
    dragAndDrop,
    settingsFile,
    loadWindow,
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
    async mounted(){
        await tauriListen("encode://count_parts", this.setCountParts);
        await tauriListen("encode://progress", this.setPrecent);
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
