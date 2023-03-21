<template>
    <div class="wrap-grid">
        <div class="upload-file-wrap">
            <dragAndDrop></dragAndDrop>
        </div>
        <div class="options-wrap">
            <settingsFile></settingsFile>
        </div>
    </div>
</template>

<script>
import dragAndDrop from './dragAndDrop.vue';
import settingsFile from './settingsFile.vue';
export default {
    name:"wrap-grid",
    components:{
        dragAndDrop,
        settingsFile,
    }
}
</script>

<style scoped>
.wrap-grid{
    display: flex;
    justify-content: space-around;
    flex-wrap: wrap;
    width: 100%;
    height: 100vh;
    padding: 20px;
}
.upload-file-wrap, .options-wrap{
    display: flex;
    align-items: flex-start;
    width: 48%;
    margin: 10px;
}
@media (max-width: 900px){
    .upload-file-wrap, .options-wrap{
        width: 100%;
    }
}
</style>