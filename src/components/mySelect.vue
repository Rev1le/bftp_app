<template>
  <div class="v-select">
    <select @change="changeOption">
      <option
        v-for="(option, index) in options"
        :key="option.value"
        :value="option.value"
        :selected="index === 0"
      >
        {{ option.name }}
      </option>
    </select>
  </div>
</template>

<script>
export default {
  name: "MySelect",
  props: {
    // modelValue: {
    //   type: String,
    // },
    options: {
      type: Array,
      default: () => [],
    },
  },
  methods: {
    changeOption(event) {
      this.$emit("update:modelValue", event.target.value);
    },
  },
};
</script>

<style>
select {
  background-color: white;
  font-size: 16px;
  padding: 10px 20px;
  border-radius: 5px;
  border: 1px solid #5989e9;
  outline: none;
  height: 40px;
  color: #5989e9;
  text-align: center;
}
option{
  display: flex;
  /* text-align: center; */
  align-items: center;
}

</style>
