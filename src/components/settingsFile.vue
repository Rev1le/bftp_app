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
        class="range"
        type="range"
        min="0"
        :max="options[dimension].max"
        :step="options[dimension].step"
        v-model.number="partWeight"
        style="width: 100%"
      />
    </div>
    <div class="part-text">
      одна часть будет равна
      <span class="part-text-big">{{ partWeight }}</span>

      {{ options[dimension].name2 }}
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
import ButtonsMix from "../mixins/ButtonsMix";
import twoButtons from "./twoButtons.vue";

export default {
  mixins: [ButtonsMix],
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
      if (!this.partWeight) {
        alert("Часть должна быть больше нуля!");
        return;
      }
      if (this.fileName == "" || !this.getNewDirectory) {
        alert("Заполните все поля");
        return;
      }
      let args = {
        filePath: this.fileName,
        options: {
          path_for_save: this.getNewDirectory,
          count_parts: null,
          part_size: this.partWeight * 1024 * 1024,
          compressed: null,
        },
      };
      if (this.dimension == 0) {
        this.$emit("splitFileRun", args);
      } else {
        args.options.part_size = args.options.part_size * 1024;
        this.$emit("splitFileRun", args);
      }
      this.newDirectory = "";
      this.partWeight = 0;
    },
  },
  computed: {
    getOpacity() {
      if (this.dimension == 0) {
        return this.partWeight / 1024 - 0.2;
      } else if (this.dimension != 0) {
        return this.partWeight / 10 - 0.2;
      }
      return 0;
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
  font-size: 1.4em;
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
