<template>
  <div
    @click="openFile"
    class="dropzone"
    :class="{ 'active-dropzone': active }"
  >
    <font-awesome-icon class="upload-icon" icon="fa-solid fa-upload" />
    <span>Перенесите файл</span>
    <span>ИЛИ</span>
    <button>Выберете файл</button>
  </div>
</template>

<script>
export default {
  props: {
    active: { type: Boolean, default: false },
    iconDrop: { type: Boolean, default: false },
    
  },
  methods: {
    openFile() {
      this.$emit("openFile");
    },
  },
};
</script>

<style scoped>

.dropzone {
  position: relative;
  cursor: pointer;
  width: 100%;
  height: 100%;
  max-height: 500px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  row-gap: 16px;
  border: 3px dashed #5989e9;
  border-radius: 10px;
  transition: all ease 0.3s;
}
.upload-icon {
  font-size: 80px;
  transition: all ease 0.3s;
  position: absolute;
  opacity: 0;
}

span {
  transition: all ease 0.3s;
  font-size: 1.6rem;
}

.active-dropzone {
  border: 3px dashed white;
  background-color: #5989e9;
}
.active-dropzone span,
.active-dropzone button {
  opacity: 0;
  background-color: #5989e9;
}

.active-dropzone .upload-icon {
  opacity: 1;
  background-color: #5989e9;
  color: white;
}
</style>
