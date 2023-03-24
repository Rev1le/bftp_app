<template>
  <transition name="v-dialog">
    <div class="dialog" v-if="show">
      <div class="dialog-content">
        <slot> </slot>
      </div>
    </div>
  </transition>
  <transition name="back">
    <div v-if="show" class="back"></div>
  </transition>
</template>

<script>
export default {
  name: "my-dialog",
  props: {
    show: {
      type: Boolean,
      default: false,
    },
  },
};
</script>

<style scoped>
.dialog {
  z-index: 10;
  top: 0;
  bottom: 0;
  right: 0;
  left: 0;
  /* background-color: rgba(0, 0, 0, 0.3); */
  background: none;
  /* transition: background-color 0.5s ease; */
  position: fixed;
  display: flex;
}
.dialog-content {
  margin: auto;
  /* background:#101112; */
  border-radius: 12px;
  min-height: 50px;
  min-width: 300px;
  max-width: 40%;
  padding: 20px;
}

.back {
  position: fixed;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  width: 100%;
  top: 0;
  left: 0;
  z-index: 9;
  /* transition: 0.1s ease-in-out; */
}

.back-enter-active,
.back-leave-active {
  transition: opacity 0.3s ease;
}
.back-enter-from,
.back-leave-to {
  opacity: 0;
}

.v-dialog-enter-active,
.v-dialog-leave-active {
  transition: all 0.3s ease;
}

.v-dialog-enter-from,
.v-dialog-leave-to {
  opacity: 0;
  transform: translateY(-300px);
}
</style>
