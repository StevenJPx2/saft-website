<template>
  <div id="outer">
    <nav id="navbar">
      <div class="navbar-inner">
        <img
          id="typeform"
          src="@/assets/saft-typeform.svg"
          alt="SAFT Logo"
          class="mb-2"
        />
        <button
          id="hamburger"
          :class="{ focused: navBarFocused }"
          @click="navBarFocused = !navBarFocused"
        >
          <svg width="30px" height="25px" viewBox="0 0 30 25">
            <rect
              id="rect-1"
              x="0"
              y="0"
              width="30"
              height="4"
              rx="1.95652174"
            ></rect>
            <rect
              id="rect-2"
              x="5.2173913"
              y="10.4347826"
              width="24.7826087"
              height="4"
              rx="1.95652174"
            ></rect>
            <rect
              id="rect-3"
              x="13.0434783"
              y="20.8695652"
              width="16.9565217"
              height="4"
              rx="1.95652174"
            ></rect>
          </svg>
        </button>
      </div>
      <ul class="links" :class="{ appear: navBarFocused }">
        <li :class="{ active: this.$store.state.pageNo == 0 }">
          <nuxt-link to="/">Home</nuxt-link>
        </li>
        <li :class="{ active: this.$store.state.pageNo == 1 }">
          <nuxt-link to="/about">About</nuxt-link>
        </li>
        <li :class="{ active: this.$store.state.pageNo == 2 }">
          <nuxt-link to="/podcast">Podcast</nuxt-link>
        </li>
        <li class="patreon-link">
          <a href="https://www.patreon.com/saftapologetics" target="_blank">
            Support Us <icon class="ml-1" name="patreon" type="fab" />
          </a>
        </li>
      </ul>
    </nav>
    <div style="height: 100px"></div>

    <Nuxt />

    <footer>
      <div id="footer">
        <div id="info">
          <h3>SAFT APOLOGETICS</h3>
          <a href="https://www.instagram.com/saftapologetics/"
            ><icon type="fab" name="instagram"
          /></a>
          <a href="https://www.youtube.com/channel/UCBDroMQT6UM9RCK3vjdW6dA/"
            ><icon type="fab" name="youtube"
          /></a>
          <a href="https://www.facebook.com/saftapologetics/"
            ><icon type="fab" name="facebook"
          /></a>
          <a href="mailto:info@saftapologetics.com"><icon name="envelope" /></a>
          <p>
            Copyright © 2020<br />
            All Rights Reserved by SAFT
          </p>
        </div>
        <li class="sitemap">
          <ul>
            <nuxt-link to="/">Home</nuxt-link>
          </ul>
          <ul>
            <nuxt-link to="/about">About</nuxt-link>
          </ul>
          <ul>
            <nuxt-link to="/podcast">Podcast</nuxt-link>
          </ul>
          <ul class="patreon-link">
            <a href="https://www.patreon.com/saftapologetics" target="_blank">
              <icon class="mr-2" name="patreon" type="fab" />Support Us
            </a>
          </ul>
        </li>
      </div>
    </footer>
  </div>
</template>

<script>
import { gsap } from "gsap";
import { ScrollTrigger } from "gsap/dist/ScrollTrigger";

export default {
  methods: {},

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
    @apply z-40;

    #typeform {
      @apply pl-3;
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
    @apply py-3;
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
    transform: translateZ(0px);

    svg {
      @apply fill-current;
    }

    #rect-1 {
      animation: rect-1-rev 1s cubic-bezier(0.86, 0, 0.07, 1);
    }

    #rect-2 {
      transition: all 0.5s cubic-bezier(0.755, 0.05, 0.855, 0.06);
    }

    #rect-3 {
      animation: rect-3-rev 1s cubic-bezier(0.86, 0, 0.07, 1);
    }

    &.focused {
      #rect-1 {
        animation: rect-1 1s cubic-bezier(0.86, 0, 0.07, 1);
        transform: translateY(90%) translateX(0%) rotate(-45deg);
      }
      #rect-2 {
        @apply opacity-0;
        @apply scale-50;
      }
      #rect-3 {
        animation: rect-3 1s cubic-bezier(0.86, 0, 0.07, 1);
        transform: translateY(-50%) translateX(57%) rotate(45deg);
        width: 30px;
        x: 0;
      }
    }
  }
}

@keyframes rect-1-rev {
  from {
    transform: translateY(90%) translateX(0%) rotate(-45deg);
  }
}

@keyframes rect-3-rev {
  from {
    transform: translateY(-50%) translateX(57%) rotate(45deg);
    width: 30px;
    x: 0;
  }
}

@keyframes rect-1 {
  0% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(50%);
  }
}
@keyframes rect-3 {
  0% {
    transform: translateY(0);
    width: 16.9565217px;
    x: 13.0434783px;
  }
  50% {
    transform: translateY(-49%);
    width: 30px;
    x: 0;
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
