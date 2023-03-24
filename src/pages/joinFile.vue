<template>
  <div class="pad join" style="">
    <div class="conten-center">
      <dragAndDrop></dragAndDrop>
    </div>
    <button>Собрать файл!</button>
  </div>
</template>

<script>
import dragAndDrop from "../components/dragAndDropMeta.vue";
export default {
  components: {
    dragAndDrop,
  },
};
</script>

<style scoped>
.conten-center {
  display: flex;
  align-items: stretch;
  justify-content: center;
  height: 90%;
  width: 100%;
}

.join {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
}

button {
  justify-self: center;
  width: 300px;
}
</style>
