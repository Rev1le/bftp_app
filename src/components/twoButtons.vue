<template>
  <div class="btn-wrap">
    <div class="settings-newPath">
      <button @click="callbackOne">{{ btnOne }}</button>
      <span>{{ getNewDirectory }}</span>
    </div>

    <div class="settings-confrim">
      <button @click="callbackTwo">{{ btnTwo }}</button>
    </div>
  </div>
</template>

<script>
export default {
    name:"two-buttons",
    props:{
        btnOne:"",
        btnTwo:"",
        getNewDirectory:"",
        callbackOne:{
            type: Function,
        },
        callbackTwo:{
            type: Function,
        },
    }
};
</script>

<style scoped>
.btn-wrap {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  width: 100%;
}

.btn-wrap > *{
  background: none;
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
</style>
