<template>
  <div>
    <nav id="navbar">
      <img
        id="typeform"
        src="@/assets/saft-typeform.svg"
        alt="SAFT Logo"
        class="mb-2"
      />
      <a href="#" id="hamburger" @click="hamburgerClick()">
        <img src="@/assets/hamburger-icon.svg" alt="Menu" />
      </a>
      <li class="links hidden">
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

<script>
export default {
  methods: {
    hamburgerClick() {
      const el = document.querySelector(".links");
      el.classList.toggle("flex");
      el.classList.toggle("hidden");
    },
  },
};
</script>

<style lang="scss" scoped>
#navbar {
  @apply px-6;
  @apply py-4;
  @apply flex;
  @apply flex-wrap;
  @apply items-center;
  @apply shadow-lg;

  background-color: #001f6d;

  #typeform {
    width: 237px;
  }

  #hamburger {
    @apply ml-auto;
  }

  .links {
    @apply flex-col;
    @apply text-white;
    @apply my-3;

    height: max-content;
    width: max-content;

    ul {
      @apply mr-5;
      @apply py-2;

      @apply rounded-sm;
      @apply uppercase;
      @apply font-bold;
      @apply tracking-widest;

      &#patreon-link {
        background-color: #f76754;
        @apply mr-0;
        @apply mt-2;
        @apply px-6;
      }
    }
  }
}

@screen md {
  #navbar {
    #typeform {
      width: 358px;
    }

    #hamburger {
      @apply hidden;
    }

    .links {
      @apply my-0;
      @apply flex-row;
      @apply ml-auto;

      ul {
        @apply px-6;

        &#patreon-link {
          @apply mt-0;
        }
      }
    }
  }
}
</style>
