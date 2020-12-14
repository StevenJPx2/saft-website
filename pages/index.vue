<template>
  <div>
    <div class="hero main-header">
      <div class="container text-white">
        <div
          id="main-circle"
          class="rounded-full shadow-xl light-gradient"
        ></div>
        <div
          id="main-circle-2"
          class="rounded-full shadow-xl dark-gradient"
        ></div>
        <h1 class="relative w-5/6 md:w-1/2">SEEKING ANSWERS, FINDING TRUTH.</h1>
      </div>
    </div>
    <section class="container-light">
      <div class="container"></div>
    </section>
  </div>
</template>

<script>
export default {};
</script>

<style lang="scss" scoped>
.main-header {
  @apply flex;
  @apply relative;
  background-color: #001f6d;
  height: 87vh;
  @apply items-center;
  @apply overflow-x-hidden;

  #main-circle {
    @apply absolute;
    width: 420px;
    height: 420px;
    left: 286px;
    top: 75px;
  }

  #main-circle-2 {
    @apply absolute;
    width: 132px;
    height: 132px;
    left: 160px;
    top: 378px;
  }
}

@screen md {
  .main-header {
    height: 735px;

    #main-circle {
      left: 350px;
      top: 90px;
    }

    #main-circle-2 {
      left: 270px;
      top: 440px;
    }
  }
}

@screen lg {
  .main-header {
    #main-circle {
      left: 770px;
      top: 90px;
    }

    #main-circle-2 {
      left: 660px;
      top: 440px;
    }
  }
}
</style>
