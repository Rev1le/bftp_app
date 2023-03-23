<template>
    <div data-tauri-drag-region  class="Navbar-vue">
        
        <img @click="$router.push('/')" class="nav_img" src="@/assets/logo.png" alt="">
      
      <div class="navbar_btns">
          <button class="btn" @click="$router.push('/')" style="margin-right: 10px;">Разбить файл</button>
          <button class="btn" @click="$router.push('/join')" style="margin-right: 10px;">
            Собрать файл
          </button>
      </div>
      <div class="window_btns">
        <font-awesome-icon @click="minimize" class="window_btn" :icon="['fas', 'window-minimize']" />
        <font-awesome-icon @click="maximize" class="window_btn" :icon="['fas', 'window-maximize']" />
        <font-awesome-icon @click="close" class="window_btn" :icon="['fas', 'rectangle-xmark']" />
      </div>
    </div>
  </template>
  
  <script>
  import { appWindow } from '@tauri-apps/api/window'
  export default{
    methods:{
        minimize(){
            appWindow.minimize();
        },
        maximize(){
            appWindow.toggleMaximize()
        },
        close(){
            appWindow.close()
        }

    }
  }
  </script>
  
  <style scoped>

  .nav_img{
    width: 32px;
  }
  
  .navbar_btns {
    display: flex;
    align-items: center;
    margin: 0 auto 0 20px;
  }
  .window_btn{
    margin-left: 15px;
    color:#5989e9;
    transition: all 0.3s ease;
  }

  .window_btn:hover{
    color: #456ab4;
  }

  .window_btns{
    display: flex;
    align-items: center;
    font-size: 15px;
  }
  
  .Navbar-vue {
    z-index: 10;
    display: flex;
    padding: 0 20px 0 0;
    align-items: center;
    height: 50px;
    align-items: center;
    box-shadow: rgba(77, 77, 77, 0.5);

  }
  
.btn {
    padding: 0;
    border: none;
    font-size: 15px;
    background: none;
    font-weight: bold;
    color:#5989e9;
  }
  
  </style>