<template>
  <div class="wrap-grid pad">
    <MyDialog :show="isLoading">
      <h1 style="text-align: center;">Файл разбивается, пожалуйста подождите. <br />(►__◄)</h1>
    </MyDialog>
    <dragAndDrop v-model:fileName="fileName" ></dragAndDrop>
    <div class="options-wrap">
      <settingsFile v-model:fileName="fileName" @showModal="showModal"></settingsFile>
    </div>
  </div>
</template>

<script>
import MyDialog from "./MyDialog.vue";
import dragAndDrop from "./dragAndDrop.vue";
import settingsFile from "./settingsFile.vue";
export default {
  name: "wrap-grid",
  components: {
    dragAndDrop,
    settingsFile,
    MyDialog,
  },
  data(){
    return{
        fileName : {
          type: String,
          default:"",
        },
        isLoading : false,
    }
  },
  methods:{
    showModal(){
      this.isLoading = !this.isLoading;
    }
  }
};
</script>

<style scoped>
.wrap-grid {
  display: flex;
  justify-content: space-around;
  flex-wrap: wrap;
}



</style>
