<template>
  <div class="shell">
    <div class="bar" :style="{ width: progress + '%' }">
      <span>{{ progress }}%</span>
    </div>
  </div>
</template>

<script>
export default {
    data(){
        return{
            progress:50,
        }
    }
}
</script>

<style scoped>
.shell {
  height: 20px;
  width: 350px;
  border: 1px solid #aaa;
  border-radius: 13px;
  box-sizing: content-box;
  padding: 3px;
 
}

.bar {
    background: linear-gradient(270deg, #b993d6, #8ca6db);
    background-size: 200% 200%;
  height: 20px;
  width: 15px;
  border-radius: 9px;
  transition: all 0.5s ease;
  padding: 0;
  -webkit-animation: barberpole 1.5s ease infinite;
    -moz-animation: barberpole 1.5s ease infinite;
    animation: barberpole 1.5s ease infinite;
}
.bar span {
    background: none;
    float: right;
    padding: 4px 5px;
    color: #fff;
    font-size: 0.7em
  }
  @-webkit-keyframes barberpole {
    0%{background-position:0% 50%}
    50%{background-position:100% 50%}
    100%{background-position:0% 50%}
}
@-moz-keyframes barberpole {
    0%{background-position:0% 50%}
    50%{background-position:100% 50%}
    100%{background-position:0% 50%}
}
@keyframes barberpole {
    0%{background-position:0% 50%}
    50%{background-position:100% 50%}
    100%{background-position:0% 50%}
}  
</style>