<template>
  <div
    class="settings"
    :style="{
      background:
        'linear-gradient(to top right, white, rgba(89, 137, 233, ' +
        getOpacity +
        '))',
    }"
  >
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
    <twoButtons
      btnOne="Выберете директорию"
      btnTwo="Разбить файл!"
      :callbackOne="newDirFile"
      :callbackTwo="confrimSettings"
      :getNewDirectory="this.getNewDirectory"
    ></twoButtons>

  </div>
</template>

<script>
import mySelect from "./mySelect.vue";
import ButtonsMix from '../mixins/ButtonsMix'
import { invoke } from "@tauri-apps/api";
import twoButtons from "./twoButtons.vue";
export default {
  mixins:[ButtonsMix],
  components: {
    mySelect,
    twoButtons,
  },
  props: {
    fileName: "",
  },
  data() {
    return {
      
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
    
    async confrimSettings() {
      if (this.partWeight) {
        if (!this.dimension) {
          await invoke("encode_file", {
            filePath: this.fileName,
            pathForSave: this.getNewDirectory,
          }).finally(()=>{
            // this.$emit("update:fileName", '')
            this.newDirectory = '';
            this.partWeight = 0;
          }
          );

          console.log(this.partWeight * 1024 * 1024, this.fileName);
        } else {
          console.log(this.partWeight * 1024 * 1024 * 1024, this.fileName);
        }
      } else {
        alert("Часть должная быть больше нуля!");
      }
    },
  },
  computed: {
    
    getOpacity() {
      if (this.dimension == 0) {
        return this.partWeight / 1024 - 0.3;
      } else if (this.dimension != 0) {
        return this.partWeight / 10 - 0.3;
      } else {
        return 0;
      }
    },
  },
  watch: {
    dimension(val) {
      this.partWeight = 0;
      // console.log("выбор ",this.dimension,this.partWeight,"WATCH")
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
.settings > *,
.part-text,
.part-text-big,
.v-select,
span {
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
  color: #5989e9;
}

input:hover {
  cursor: grab;
}
</style>
