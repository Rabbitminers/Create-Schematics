<template>
    <img :src="src" alt="Logo" />
</template>

<script lang="ts">
export default {
  props: {
    size: {
      type: String,
      default: "m",
      validator(value: string) {
        return ["xs", "s", "m", "l", "xl"].includes(value);
      },
    },
  },
  data() {
    return {
      src: `/logo/${this.size}.png`,
    };
  },
};
</script>
