<template>
  <font-awesome-icon :icon="[type, name]"></font-awesome-icon>
</template>

<script>
import Vue from "vue";
export default Vue.extend({
  props: {
    name: String,
    type: {
      type: String,
      default: "fas",
    },
  },
});
</script>
