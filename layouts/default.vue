<template>
  <div id="outer">
    <nav id="navbar">
      <div class="navbar-inner">
        <img
          id="typeform"
          src="@/assets/saft-typeform.svg"
          alt="SAFT Logo"
          class=""
        />
        <button
          id="hamburger"
          :class="{ focused: navBarFocused }"
          @click="toggleHamburger()"
        >
          <svg width="30px" height="25px" viewBox="0 0 30 25">
            <rect id="rect-1" x="0" y="0" width="30" height="4" rx="2"></rect>
            <rect id="rect-2" x="6" y="10" width="24" height="4" rx="2"></rect>
            <rect id="rect-3" x="14" y="20" width="16" height="4" rx="2"></rect>
          </svg>
        </button>
      </div>
      <ul class="links" :class="{ appear: navBarFocused }">
        <li
          :class="{ active: this.$store.state.pageNo == 0 }"
          @click="toggleHamburger()"
        >
          <nuxt-link to="/">Home</nuxt-link>
        </li>
        <li
          :class="{ active: this.$store.state.pageNo == 1 }"
          @click="toggleHamburger()"
        >
          <nuxt-link to="/about">About</nuxt-link>
        </li>
        <li
          :class="{ active: this.$store.state.pageNo == 2 }"
          @click="toggleHamburger()"
        >
          <nuxt-link to="/podcast">Podcast</nuxt-link>
        </li>
        <li>
          <a href="https://blog.saftapologetics.com" target="_blank">Blog</a>
        </li>
        <li>
          <a href="https://areopagus.saftapologetics.com" target="_blank"
            >Areopagus</a
          >
        </li>
      </ul>
    </nav>
    <div style="height: 100px"></div>
    <div
      class="fixed inset-0 z-30"
      :class="{ hidden: !navBarFocused }"
      @click="toggleHamburger()"
    ></div>

    <Nuxt />

    <footer>
      <div id="footer">
        <div id="info">
          <h3>SAFT APOLOGETICS</h3>
          <a href="https://www.instagram.com/saftapologetics/">
            <icon type="fab" name="instagram" />
          </a>
          <a href="https://www.youtube.com/channel/UCBDroMQT6UM9RCK3vjdW6dA/">
            <icon type="fab" name="youtube" />
          </a>
          <a href="https://www.facebook.com/saftapologetics/">
            <icon type="fab" name="facebook" />
          </a>
          <a href="mailto:info@saftapologetics.com">
            <icon name="envelope" />
          </a>
          <p>
            Copyright © 2020<br />
            All Rights Reserved by SAFT
          </p>
        </div>
        <ul class="sitemap">
          <li>
            <nuxt-link to="/">Home</nuxt-link>
          </li>
          <li>
            <nuxt-link to="/about">About</nuxt-link>
          </li>
          <li>
            <nuxt-link to="/podcast">Podcast</nuxt-link>
          </li>
          <li>
            <a href="https://blog.saftapologetics.com" target="_blank">Blog</a>
          </li>
          <li>
            <a href="https://areopagus.saftapologetics.com" target="_blank"
              >Areopagus</a
            >
          </li>
        </ul>
      </div>
    </footer>
  </div>
</template>

<script>
import { gsap } from "gsap";
import { ScrollTrigger } from "gsap/dist/ScrollTrigger";

export default {
  methods: {
    toggleHamburger() {
      this.navBarFocused = !this.navBarFocused;
      let ease = "expo.inOut";

      if (this.navBarFocused) {
        gsap.to("#rect-1", {
          keyframes: [
            { y: 10, ease: ease },
            {
              rotation: -45,
              y: 20,
              x: 5,
              transformOrigin: "50% left",
              ease: ease,
              duration: 0.7,
            },
          ],
        });

        gsap.to("#rect-2", {
          opacity: 0,
        });

        gsap.to("#rect-3", {
          keyframes: [
            { y: -10, scaleX: 1.875, x: -14, ease: ease },
            {
              rotation: 45,
              y: 0,
              x: -17,
              transformOrigin: "50% right",
              ease: ease,
              duration: 0.7,
            },
          ],
        });
      } else {
        let duration = 0.8;
        gsap.to("#rect-1", {
          keyframes: [
            {
              y: 0,
              rotation: 0,
              x: 0,
              ease: ease,
              duration: duration,
            },
          ],
        });
        gsap.to("#rect-2", {
          opacity: 1,
          duration: duration,
        });
        gsap.to("#rect-3", {
          keyframes: [
            {
              y: 0,
              rotation: 0,
              x: -14,
              scaleX: 1,
              ease: ease,
              duration: duration,
            },
          ],
        });
      }
    },
  },

  data() {
    return {
      navBarFocused: false,
    };
  },

  mounted() {},
};
</script>

<style lang="scss" scoped>
#navbar {
  @apply w-screen;
  height: 100px;
  @apply fixed;
  @apply z-50;
  @apply grid;
  @apply items-center;
  @apply shadow-lg;

  background-color: #002586;

  .navbar-inner {
    background-color: #002586;
    @apply py-4;
    @apply w-screen;
    @apply h-full;
    @apply flex;
    @apply relative;
    @apply items-center;
    @apply z-40;

    #typeform {
      @apply pl-3;
      @apply h-full;
      width: 250px;
    }
  }

  .links {
    @apply col-start-1;
    @apply col-end-3;
    @apply row-start-2;
    @apply flex;
    @apply flex-col;
    @apply text-white;
    @apply -mt-2;
    @apply pb-3;
    @apply pl-3;
    @apply w-screen;
    @apply transition-all;
    @apply ease-in-out;
    @apply duration-500;
    transform: translateY(-150%);

    background-color: #002586;

    &.appear {
      transform: translateY(0%);
      @apply opacity-100;
    }

    li {
      @apply mr-5;
      @apply py-2;
      @apply w-max;

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

  #hamburger {
    @apply relative;
    @apply z-40;
    @apply ml-auto;
    @apply pr-3;
    @apply self-center;
    @apply text-white;
    @apply outline-none;

    svg {
      @apply fill-current;
    }
  }
}

footer {
  background-color: #002586;

  #footer {
    @apply grid;
    grid-template-rows: max-content auto;
    @apply pt-8;
    @apply px-4;
    @apply mt-4;
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

      li {
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
      @apply pt-16;
      @apply px-6;
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

@screen xl {
  #navbar {
    @apply flex;
    @apply px-6;
    @apply flex-wrap;
    height: 130px;

    .navbar-inner {
      @apply w-auto;

      #typeform {
        width: 358px;
      }
    }

    #hamburger {
      @apply hidden;
    }

    .links {
      @apply flex;
      @apply self-center;
      @apply w-max;
      height: max-content;
      @apply my-0;
      @apply pb-0;
      @apply flex-row;
      @apply ml-auto;
      transform: translateY(0%);
      @apply opacity-100;

      li {
        @apply px-6;

        &.active {
          background: #1a44b2;
        }

        &.patreon-link {
          @apply mt-0;
        }
      }
    }
  }

  #footer {
    @apply px-0;
  }
}
</style>
