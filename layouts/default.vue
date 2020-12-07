<template>
  <div>
    <nav id="navbar">
      <img src="@/assets/saft-typeform.svg" alt="SAFT Logo" class="mb-2" />
      <li class="links">
        <ul>
          <a href="#">Home</a>
        </ul>
        <ul>
          <a href="#">About</a>
        </ul>
        <ul>
          <a href="#">Podcast</a>
        </ul>
        <ul id="patreon-link">
          <a href="#" target="_blank">
            Support Us <icon class="ml-1" name="patreon" type="fab" />
          </a>
        </ul>
      </li>
    </nav>
    <Nuxt />
  </div>
</template>

<style lang="scss" scoped>
#navbar {
  @apply px-6;
  @apply py-4;
  @apply flex;
  @apply items-center;
  background-color: #001f6d;

  .links {
    @apply flex;
    @apply ml-auto;
    @apply text-white;
    height: max-content;

    width: max-content;

    ul {
      @apply mr-5;
      @apply py-2;
      @apply px-6;
      @apply rounded-sm;
      @apply uppercase;
      @apply font-bold;
      @apply tracking-widest;

      &#patreon-link {
        background-color: #f76754;
        @apply mr-0;
      }
    }
  }
}
</style>
