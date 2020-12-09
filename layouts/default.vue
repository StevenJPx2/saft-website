<template>
  <div id="outer">
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
        <ul class="patreon-link">
          <a href="#" target="_blank">
            Support Us <icon class="ml-1" name="patreon" type="fab" />
          </a>
        </ul>
      </li>
    </nav>
    <Nuxt />
    <footer>
      <div id="footer">
        <div id="info">
          <h3>SAFT APOLOGETICS</h3>
          <a href="#"><icon type="fab" name="instagram" /></a>
          <a href="#"><icon type="fab" name="youtube" /></a>
          <a href="#"><icon type="fab" name="facebook" /></a>
          <a href="#"><icon name="envelope" /></a>
          <p>
            Copyright © 2020<br />
            All Rights Reserved by SAFT
          </p>
        </div>
        <li class="sitemap">
          <ul>
            <a href="#">Home</a>
          </ul>
          <ul>
            <a href="#">About</a>
          </ul>
          <ul>
            <a href="#">Podcast</a>
          </ul>
          <ul class="patreon-link">
            <a href="#" target="_blank">
              <icon class="mr-2" name="patreon" type="fab" />Support Us
            </a>
          </ul>
        </li>
      </div>
    </footer>
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
#outer {
  @apply grid;
  grid-template-rows: auto 1fr auto;
  @apply h-screen;
}

#navbar {
  @apply px-6;
  @apply py-4;
  @apply grid;
  @apply items-center;
  @apply shadow-lg;
  grid-template-columns: auto auto;

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

      &.patreon-link {
        background-color: #f76754;
        @apply mr-0;
        @apply mt-2;
        @apply px-6;
      }
    }
  }
}

footer {
  background-color: #001f6d;

  #footer {
    @apply grid;
    grid-template-rows: max-content auto;
    @apply mx-3;
    @apply my-4;
    height: 640px;
    @apply text-white;

    #info {
      width: Min(300px, 100%);

      h3 {
        @apply text-3xl;
        @apply leading-none;
        @apply font-bold;
        @apply mb-6;
      }

      a {
        @apply text-3xl;
        @apply mr-8;
      }

      p {
        @apply mt-6;
        @apply text-sm;
      }
    }

    .sitemap {
      @apply row-start-1;
      @apply list-none;
      @apply font-medium;

      ul {
        @apply mb-4;

        &.patreon-link {
          @apply pt-2;
          @apply pb-12;

          a {
            background-color: #f76754;
            @apply w-max;
            @apply rounded-sm;
            @apply px-4;
            @apply py-2;
          }
        }
      }
    }
  }
}

@screen md {
  footer {
    @apply grid;
    @apply place-items-center;

    #footer {
      // @apply place-items-center;
      @apply pt-16;
      height: 500px;
      @apply grid-cols-2;
      width: Min(1100px, 100%);

      .info {
        @apply col-start-1;
      }

      .sitemap {
        @apply text-right;
        @apply col-start-2;
      }
    }
  }
}

@screen lg {
  #navbar {
    @apply flex;
    @apply flex-wrap;

    #typeform {
      width: 358px;
    }

    #hamburger {
      @apply hidden;
    }

    .links {
      display: flex !important;
      @apply my-0;
      @apply flex-row;
      @apply ml-auto;

      ul {
        @apply px-6;

        &.patreon-link {
          @apply mt-0;
        }
      }
    }
  }
}
</style>
