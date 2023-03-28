<template>
  <MyDialog :show="show">
    <div class="center-content">
      <h1 style="text-align: center; margin-bottom: 20px">
        Файл разбивается, пожалуйста подождите.
      </h1>
      <img class="cat" src="../assets/cat.gif" alt="" />
      <progressBar :progress="precent"></progressBar>
    </div>
  </MyDialog>
</template>

<script>
import MyDialog from "./MyDialog.vue";
import progressBar from "./progressBar.vue";
export default {
  props: {
    precent: Number,
    show: {
      type: Boolean,
      default: false,
    },
  },
  components: {
    MyDialog,
    progressBar
  },
};
</script>

<style scoped>
.center-content{
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
.cat{
    width: 100%;
    max-width: 300px;
}
</style>
