<template>
  <div class="settings" :style="{'background': 'linear-gradient(to top right, white, rgba(89, 137, 233, '+ getOpacity +'))'}">
    <div class="settings-part">
      <mySelect :options="options" v-model="dimension"></mySelect>
      
      <input
        type="range"
        min="0"
        :max="options[dimension].max"
        :step="options[dimension].step"
        v-model.number="partWeight"
        style="width: 100%"
      />

      <div class="part-text">
        одна часть будет равна
        <span class="part-text-big">{{ partWeight }}</span>
        {{ options[dimension].name2 }}
      </div>
    </div>

    <div class="settings-newPath">
      <button @click="newDirFile">Выберете директорию</button>
      <span>{{ getNewDirectory }}</span>
    </div>

    <div class="settings-confrim">
      <button @click="confrimSettings">Разбить файл!</button>
    </div>
  </div>
</template>

<script>
import mySelect from "./mySelect.vue";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from '@tauri-apps/api'
export default {
  components: {
    mySelect,
  },
  props:{
    fileName:'',
  },
  data() {
    return {
      newDirectory: "",
      partWeight: 0,
      dimension: 0,
      options: [
        {
          value: 0,
          name: "Мегабайты",
          name2: "Мегабайт",
          step: "1",
          max: "1024",
        },
        {
          value: 1,
          name: "Гигабайты",
          name2: "Гигабайт",
          step: "0.25",
          max: "10",
        },
      ],
    };
  },
  methods: {
    async newDirFile() {
      this.newDirectory = await open({
        directory: true,
        multiple: true,
        defaultPath: "/",
      });
      console.log(this.newDirectory);
    },
    async confrimSettings() {
      if (!this.dimension) {
        console.log(this.partWeight * 1024 * 1024, this.fileName);
        await invoke("encode_file",{filePath:this.fileName,pathForSave:this.newDirectory})
      } else {
        console.log(this.partWeight * 1024 * 1024 * 1024, this.fileName);
      }
    },
  },
  computed: {
    getNewDirectory() {
      try {
        return this.newDirectory[0];
      } catch {
        return this.newDirectory;
      }
    },
    getOpacity(){
      if(!this.dimension && this.partWeight !== 0){
        return this.partWeight/1024 -0.4
      }
      else if (this.partWeight !== 0) {
        return this.partWeight/10 -0.4
      }
      else {
        return 0
      }
    },
  
  },
  watch: {
    dimension(val) {
      this.partWeight = 0;
    },
  },
};
</script>

<style scoped>
.settings {
  padding: 15px;
  border-radius: 5px;
  box-shadow: 0px 4px 4px 4px rgba(0, 0, 0, 0.2);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  width: 100%;
  height: 100%;
  max-height: 500px;
  
  
}
.settings > *, .part-text, .part-text-big, .v-select, span{
    background: none;
}
.settings-part {
  font-size: 1.4em;
}
.part-text {
  display: flex;

  width: 100%;
  justify-content: space-evenly;
  flex-direction: column;
  align-items: center;
}
.part-text-big {
  font-size: 4em;
  line-height: 80px;
  color:#5989e9;
}
.settings-newPath,
.settings-confrim {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.settings-newPath span {
  color: #5989e99a;
  font-size: 14px;
}
.settings-newPath button,
.settings-confrim button {
  margin-bottom: 1%;
  width: 50%;
}
input:hover {
  cursor: grab;
}
</style>
