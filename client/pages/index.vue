<template>
  <div class="container">
    <h1 class="main-header">
      Seeking
      <br />
      answers, <br />finding <br />truth.
    </h1>
    <div id="head-oval" class="blue-oval"></div>
    <div class="content"><p>Hello</p></div>
  </div>
</template>

<script>
export default {
  name: 'HomePage',
}
</script>

<style lang="scss" scoped>
.container {
  overflow-x: clip;

  .main-header {
    @apply uppercase;
    @apply leading-none;
    @apply font-black;
    font-size: Min(15vw, 5rem);
  }

  #head-oval {
    @apply relative;
    left: 50%;
    top: -10%;
    width: Min(400px, 60vw);
    height: Min(400px, 60vw);
  }
}

@screen md {
  .container {
    overflow-x: initial;
  }
}
</style>
