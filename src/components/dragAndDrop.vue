<template>
  <div class="upload-file-wrap">
    <dragAndDropItem
      :active="active"
      :iconDrop="iconDrop"
      @openFile="openFile"
    ></dragAndDropItem>
    <div class="files" :class="{ 'active-files': iconDrop }" >
      <font-awesome-icon class="upload-icon-drop" :icon="['far', 'file']" />
      <span class="upload-icon-drop">{{ fileName }}</span>
      <font-awesome-icon
        @click="closeFile"
        class="upload-icon-drop"
        :icon="['fas', 'xmark']"
        style="cursor: pointer"
      />
    </div>
  </div>
</template>

<script>
import dragAndDropItem from "./dragAndDropItem.vue";
import dAndDMix from "../mixins/mixinDragAndDrop";

export default {
  mixins:[dAndDMix],
  components: {
    dragAndDropItem,
  },
  watch:{
    fileName(val){
      if (val!=""){
        this.iconDrop=true;
      }
      else{
        this.iconDrop=false;
      }
    }
  }
 
};
</script>

<style scoped>
.files {
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;
  margin:5px 0 5px 0;
  height: 0;
  transition: all 0.7s ease;
  opacity: 0;
  transform: translateX(-150px);
}
.files * {
  margin-right: 10px;
}
.active-files{
  transform: translateX(0px);
  height: auto;
  opacity: 1;
  transition: all 0.7s ease;
}

.upload-icon-drop {
  font-size: 20px;
  transition: all ease 0.3s;
  color: #5989e9;
}
span {
  word-break: break-word;
}
</style>
