<template>
  <input
        type="range"
        min="0"
        :max="maxStep"
        :step="step"
        v-model.number="inputVal"
        @mousemove="changeRange"
        style="width: 100%; margin-bottom: 5%;"
      />
      <!-- <span>{{ inputVal}}</span> -->
</template>

<script>
export default {
    data(){
        return{
            inputVal:0,
        }
    },
    props:{
        step:{type:String,
            default:'1',},
        maxStep:{
            type:String,
            default:'1024',
        },
        number:{
            type:Number,
            default:0,
        }
    },
    methods:{
        changeRange(){
            // console.log(this.inputVal)
            this.$emit("update:number", this.inputVal);
        }
    },
    mounted(){
        this.inputVal = this.number;
    }
    // watch:{
    //     inputVal(val){
    //         console.log(val)
    //         this.$emit("changeRange", val);
    //     }
    // }
}
</script>

<style scoped>

</style>