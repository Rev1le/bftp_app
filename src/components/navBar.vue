<template>
  <div data-tauri-drag-region class="Navbar-vue">
    <img
      class="nav_img"
      src="@/assets/logo.png"
      alt=""
    />

    <div class="navbar_btns">
      <input id="split" name="nav" type="radio" value="0" checked />
      <label @click="$router.push('/')" class="btn" for="split"
        >Разбить файл</label
      >

      <input id="join" name="nav" type="radio" value="1" />
      <label @click="$router.push('/join')" class="btn" for="join"
        >Собрать файл</label
      >
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
// import { event } from "@tauri-apps/api";
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

   
  },
};
</script>

<style scoped>
input[type="radio"] {
  display: none;
}
input[type="radio"]:checked + .btn,
.btn:hover {
  border-bottom: 2px solid #5989e9;
}

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
  cursor: pointer;
}

.window_btn:hover {
  color: #456ab4;
  transform: scale(1.2);
}

.window_btn:active {
  color: #456ab4;
  transform: translateY(5px);
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
  transition: all 0.3s ease;
  text-decoration: none;
  padding: 5px;
  border-bottom: 2px solid transparent;
  font-size: 15px;
  background: none;
  font-weight: bold;
  color: #5989e9;
  margin-right: 15px;
  cursor: pointer;
}
</style>
