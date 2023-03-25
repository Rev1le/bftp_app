<template>
  <navBar></navBar>
  <router-view v-slot="{ Component, route }">
    <transition :name="route.meta.transition" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<script>
// import breakFile from './pages/breakFile.vue';
import navBar from "./components/navBar.vue";
export default {
  components: {
    // breakFile,
    navBar,
  },
};
</script>

<style>
.slide-left-enter-active,
.slide-left-leave-active,
.slide-right-enter-active,
.slide-right-leave-active {
  transition: all 0.5s ease-in-out;
}

.slide-left-enter-from,  .slide-right-leave-to  {
  transform: translateX(-40%);
  opacity: 0;
}

.slide-right-enter-from, .slide-left-leave-to {
  transform: translateX(40%);
  opacity: 0;
}

</style>
