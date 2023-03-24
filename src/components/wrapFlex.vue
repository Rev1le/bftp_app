<template>
  <div class="wrap-grid pad">
    <dragAndDrop v-model:fileName="fileName" ></dragAndDrop>
    <div class="options-wrap">
      <settingsFile :fileName="fileName"></settingsFile>
    </div>
  </div>
</template>

<script>
import dragAndDrop from "./dragAndDrop.vue";
import settingsFile from "./settingsFile.vue";
export default {
  name: "wrap-grid",
  components: {
    dragAndDrop,
    settingsFile,
  },
  data(){
    return{
        fileName : {
          type: String,
          default:"",
        },
    }
  },
  methods:{

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
