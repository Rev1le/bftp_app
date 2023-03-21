<template>
  <div class="settings">
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
export default {
  components: {
    mySelect,
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
        console.log(this.partWeight * 1024 * 1024);
      } else {
        console.log(this.partWeight * 1024 * 1024 * 1024);
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
    // getNewGradient() {
    //   let gradient;
    //   if (!this.dimension && this.partWeight !== 0) {
    //     gradient = `linear-gradient(to right, white, rgb(0,0,${
    //       this.partWeight/4 - 1
    //     }));`;
    //   } else if (this.partWeight !== 0) {
    //     gradient = `linear-gradient(to right, white, rgb(0,0,${
    //       this.partWeight * 25.5
    //     }));`;
    //   } else {
    //     gradient = `linear-gradient(to right, white , rgb(0,0,0));`;
    //   }
    //   return gradient;
    // },
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
  /* background-image: linear-gradient(to right, white , rgb(0,0,60)); */
}
/* .settings *{
    margin-bottom: 15px;
} */
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
