<template>
  <div class="upload-file-wrap">
    <dragAndDropItem
      :active="active"
      :iconDrop="iconDrop"
      @openFile="openFile"
    ></dragAndDropItem>
    <div
      class="files"
      :class="{ 'active-files': iconDrop }"
      :style="[iconDrop ? { minHeight: 25 + 'px' } : { height: 0 }]"
    >
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
import dAndDMix from "../mixins/dAndDMix";

export default {
  mixins: [dAndDMix],
  components: {
    dragAndDropItem,
  },
  methods: {
   
 
  },
};
</script>

<style scoped>
.files {
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;
}
.files * {
  margin-right: 10px;
}
.active-files .upload-icon-drop {
  opacity: 1;
}
.upload-icon-drop {
  font-size: 20px;
  transition: all ease 0.3s;
  opacity: 0;
  color: #5989e9;
}
span {
  word-break: break-word;
}
</style>
