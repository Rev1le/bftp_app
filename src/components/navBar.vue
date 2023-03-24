<template>
  <div data-tauri-drag-region class="Navbar-vue">
    <img
      @click="$router.push('/')"
      class="nav_img"
      src="@/assets/logo.png"
      alt=""
    />

    <div class="navbar_btns">
      <button
        class="btn"
        @click="splitFilePage"
        :class="{ 'btn-active': activeSplit }"
      >
        Разбить файл
      </button>
      <button
        class="btn"
        id="join"
        @click="splitFilePage"
        :class="{ 'btn-active': !activeSplit }"
      >
        Собрать файл
      </button>
    </div>
    <div class="window_btns">
      <font-awesome-icon
        @click="minimize"
        class="window_btn"
        :icon="['fas', 'window-minimize']"
      />
      <font-awesome-icon
        @click="maximize"
        class="window_btn"
        :icon="['fas', 'window-maximize']"
      />
      <font-awesome-icon
        @click="close"
        class="window_btn"
        :icon="['fas', 'rectangle-xmark']"
      />
    </div>
  </div>
</template>
  
  <script>
import { appWindow } from "@tauri-apps/api/window";
import { event } from "@tauri-apps/api";
export default {
  data() {
    return {
      activeSplit: true,
    };
  },
  methods: {
    minimize() {
      appWindow.minimize();
    },
    maximize() {
      appWindow.toggleMaximize();
    },
    close() {
      appWindow.close();
    },

    splitFilePage(event) {
      if (event.target.id === "join") {
        this.activeSplit = false;
      } else {
        this.activeSplit = true;
      }
      this.$router.push(`/${event.target.id}`);
    },

    joinFilePage() {
      this.$router.push("/join");
    },
  },
};
</script>
  
  <style scoped>
.nav_img {
  width: 32px;
}

.navbar_btns {
  display: flex;
  align-items: center;
  margin: 0 auto 0 20px;
}
.window_btn {
  margin-left: 15px;
  color: #5989e9;
  transition: all 0.3s ease;
}

.window_btn:hover {
  color: #456ab4;
  transform: scale(1.2);
}

.window_btns {
  display: flex;
  align-items: center;
  font-size: 20px;
}

.Navbar-vue {
  z-index: 10;
  display: flex;
  padding: 0 20px 0 15px;
  align-items: center;
  height: 50px;
  align-items: center;
 
}

.btn {
  padding: 0;
  border: 1px solid transparent;
  border-radius: 5px;
  padding: 5px 10px;
  font-size: 15px;
  background: none;
  font-weight: bold;
  color: #5989e9;
  margin-right: 15px;
}
.btn-active {
  transition: all 0.3s ease;
  border: 1px solid rgb(190, 190, 190);
}
</style>