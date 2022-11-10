<template>
  <div>
    <header class="hero main-header">
      <div class="container text-white">
        <div class="relative top-12 left-2/4 w-max">
          <div
            id="main-circle"
            class="rounded-full shadow-xl light-gradient"
          ></div>
          <div
            id="main-circle-2"
            class="relative -top-14 -left-20 rounded-full shadow-xl dark-gradient"
          ></div>
        </div>
        <h1
          class="absolute top-1/3 z-10 w-4/6 font-extrabold md:w-1/4"
          style="line-height: 3.15rem"
        >
          <div class="inline-block main-line">SEEKING</div>
          <div class="inline-block main-line">ANSWERS,</div>
          <div class="inline-block main-line">FINDING</div>
          <div class="inline-block main-line">TRUTH.</div>
        </h1>
      </div>
    </header>

    <section class="container-light">
      <div id="about-us" class="container relative">
        <div id="founder-images">
          <div
            class="card"
            v-for="{ id, name, title, description, img } in coreTeamMembers"
            :key="id"
          >
            <img
              :src="`https://5ms1k56r.directus.app/assets/${img}`"
              :alt="name"
            />
            <div class="card--details">
              <h2 class="card--details--title">{{ name }}</h2>
              <h3 class="card--details--subtitle">{{ title }}</h3>
              <p class="card--details--desc">
                {{ description.substr(0, 160) + "..." }}
              </p>
            </div>
          </div>
        </div>

        <div id="about-us--desc">
          <div aos>
            <h2 class="uppercase">Who are we?</h2>
            <p>
              {{ homePageData.who_are_we }}
            </p>
          </div>
          <br />
          <div aos>
            <h2 class="uppercase">What do we do?</h2>
            <p>{{ homePageData.what_we_do }}</p>
          </div>
          <div class="flex mt-4 w-full">
            <div class="w-full"></div>
            <nuxt-link to="/about#core-team" class="flex-none btn"
              >More<icon class="ml-2" name="arrow-right" />
            </nuxt-link>
          </div>
        </div>
      </div>
    </section>

    <section class="container relative py-24">
      <div id="tagline" class="flex items-center">
        <blockquote class="z-10 my-auto" aos>
          {{ homePageData.tagline }}
        </blockquote>
      </div>
    </section>
    <section class="container-light">
      <div class="container endorsements">
        <section class="endorsements--card" aos>
          <p class="endorsements--card--desc">
            They are building a great resource and a much-needed ministry...
          </p>
          <h3 class="endorsements--card--name">Brian Auten</h3>
          <h4 class="endorsements--card--title">
            Founder-Emeritus, Apologetics315
          </h4>
        </section>
        <section class="md:col-start-2 md:row-start-2 endorsements--card" aos>
          <p class="endorsements--card--desc">
            ...the team at SAFT are not afraid to tackle the hardest
            questions...
          </p>
          <h3 class="endorsements--card--name">Fazale 'Fuz' Rana</h3>
          <h4 class="endorsements--card--title">
            Vice President Of Research And Apologetics, Reasons To Believe
          </h4>
        </section>
        <section class="md:col-start-1 md:row-start-3 endorsements--card" aos>
          <p class="endorsements--card--desc">
            ...I highly commend them to you...
          </p>
          <h3 class="endorsements--card--name">Jerin Thomas</h3>
          <h4 class="endorsements--card--title">
            Global Mission Catalyst, WCD of the Christian and Missionary
            Alliance, Canada
          </h4>
        </section>
        <nuxt-link
          to="/about#endorsements"
          class="place-self-end mt-4 md:col-start-2 md:row-start-4 btn"
          >More<icon class="ml-2" name="arrow-right" />
        </nuxt-link>
      </div>
    </section>
    <section class="container podcast">
      <article class="podcast--title">
        <h1 class="mb-3" aos>Saft Podcast</h1>
        <p class="text-xl" aos>Listen to our latest episode here.</p>
      </article>

      <article class="podcast--preview" aos>
        <div id="buzzsprout-small-player-1034671"></div>
      </article>

      <article class="podcast--buttons">
        <a
          href="https://podcasts.apple.com/in/podcast/saft-podcast/id1511404295"
          target="_blank"
          aos
          ><img
            src="@/assets/apple-podcasts.png"
            alt="Listen on Apple Podcasts"
        /></a>
        <a
          href="https://podcasts.google.com/?feed=aHR0cHM6Ly9mZWVkcy5idXp6c3Byb3V0LmNvbS8xMDM0NjcxLnJzcw=="
          target="_blank"
          aos
          ><img
            src="@/assets/google-podcasts.png"
            alt="Listen on Google Podcasts"
        /></a>
        <a
          href="https://open.spotify.com/show/4hOLouY5QFv3KuNNDUi5hM"
          target="_blank"
          aos
          ><img src="@/assets/spotify-podcasts.png" alt="Listen on Spotify"
        /></a>
        <a
          href="https://www.youtube.com/channel/UCBDroMQT6UM9RCK3vjdW6dA/videos"
          target="_blank"
          aos
          ><img src="@/assets/youtube-podcasts.png" alt="Watch on Youtube"
        /></a>
      </article>
      <nuxt-link
        to="/podcast"
        class="place-self-end mt-4 md:col-start-2 md:row-start-4 btn"
        >More<icon class="ml-2" name="arrow-right" />
      </nuxt-link>
    </section>
  </div>
</template>

<script>
import { gsap } from "gsap";
import { ScrollTrigger } from "gsap/dist/ScrollTrigger";
import { Directus } from "@directus/sdk";

export default {
  head() {
    return {
      title: "SAFT Apologetics",
      script: [
        {
          type: "text/javascript",
          charset: "utf-8",
          src: "https://www.buzzsprout.com/1034671.js?container_id=buzzsprout-small-player-1034671&player=small&limit=1",
          body: true,
          defer: true,
        },
      ],
      meta: [
        {
          hid: "description",
          name: "description",
          content: `Equipping the believer defend their faith anytime, anywhere.`,
        },
      ],
    };
  },

  mounted() {
    this.$store.commit("page", 0);

    gsap.registerPlugin(ScrollTrigger);

    this.$store.commit("initAnimations");

    gsap.to("#main-circle", {
      keyframes: [
        { x: -5, y: -6 },
        { x: -7, y: -6.5 },
        { x: -6.5, y: -7.2 },
        { x: -5, y: -6 },
      ],
      yoyo: true,
      repeat: -1,
      ease: "sine.inOut",
    });

    gsap.to("#main-circle-2", {
      keyframes: [
        { x: -5, y: -6 },
        { x: -7, y: -6.5 },
        { x: -6.5, y: -7.2 },
        { x: -5, y: -6 },
      ],
      delay: 0.2,
      yoyo: true,
      repeat: -1,
      ease: "sine.inOut",
    });

    gsap.fromTo(
      ".main-line",
      { opacity: 0, y: 50 },
      {
        opacity: 1,
        y: 0,
        stagger: 0.1,
        duration: 1.5,
        ease: "power2.inOut",
        scrollTrigger: {
          trigger: ".main-line",
          toggleActions: "restart continue play reverse",
        },
      }
    );

    gsap.fromTo(
      "#founder-images .card",
      { opacity: 0, y: 50 },
      {
        opacity: 1,
        y: 0,
        stagger: 0.1,
        duration: 1.5,

        ease: "power2.inOut",
        scrollTrigger: {
          trigger: "#founder-images",
          toggleActions: "restart continue play reverse",
        },
      }
    );
  },

  created() {
    this.$store.commit("page", 0);
  },

  async asyncData() {
    const api = new Directus("https://5ms1k56r.directus.app/");
    const homePageData = await api.singleton("Home").read();
    const coreTeamMembers = await api.singleton("Team").read();
    const res = { homePageData, coreTeamMembers };
    console.log(res);

    return res;
  },
};
</script>

<style lang="scss" scoped>
.main-header {
  @apply flex;
  @apply relative;
  background-color: #001f6d;
  height: Max(87vh, 100%);
  @apply items-center;
  @apply overflow-hidden;

  h1 {
    font-size: Min(3.75rem, 15vw);
  }

  #main-circle {
    width: 420px;
    height: 420px;
  }

  #main-circle-2 {
    width: 132px;
    height: 132px;
  }
}

#about-us {
  @apply grid;
  @apply gap-x-12;
  grid-template-columns: auto;
}

#founder-images {
  @apply grid;
  grid-template-columns: repeat(2, minmax(120px, 210px));
  @apply relative;
  @apply gap-x-6;
  @apply place-self-center;
}

.card {
  &:nth-child(1) {
    @apply col-span-1;
    @apply row-start-1;
    @apply place-self-end;
    @apply row-span-1;
  }

  &:nth-child(2) {
    @apply my-auto;
    @apply col-start-2;
    @apply col-end-3;
    @apply row-start-1;
    @apply row-end-3;
  }

  &:nth-child(3) {
    @apply place-self-end;
    @apply col-span-1;
    @apply row-start-2;
    @apply row-end-2;
  }

  @apply mb-6;
  @apply w-max;
  height: max-content;
  @apply flex;

  img {
    @apply rounded-xl;
    @apply shadow-lg;
    width: 120px;
    height: 120px;
    @apply row-span-3;
    @apply object-cover;
  }

  &--details {
    @apply hidden;
    @apply bg-white;
    @apply shadow-lg;
    @apply rounded-r-xl;
    @apply transition-all;
    @apply duration-200;
    @apply ease-in-out;
    @apply opacity-0;
    transform: translateX(-100%);
    height: 120px;
    width: 300px;
    @apply px-5;
    @apply py-5;
    @apply overflow-hidden;

    &--title {
      @apply uppercase;
      @apply text-xl;
      @apply leading-none;
      @apply font-semibold;
    }

    &--subtitle {
      @apply text-base;
      color: #6c6c6c;
    }

    &--desc {
      @apply mt-2;
      @apply text-xs;
      @apply leading-4;
    }
  }
}

#tagline {
  background: url("../assets/quotation.svg") no-repeat;
  height: 300px;
}

.endorsements {
  @apply grid;
  @apply gap-y-12;

  &--card {
    &--desc {
      @apply font-medium;
      @apply text-xl;
      @apply mb-4;
    }
    &--name {
      @apply text-lg;
      @apply uppercase;
      @apply font-bold;
      @apply mb-1;
    }
    &--title {
      @apply text-lg;
      @apply uppercase;
      color: #555555;
      @apply leading-6;
    }
  }
}

.podcast {
  @apply grid;
  @apply py-16;
  @apply gap-y-8;

  &--preview {
    @apply w-full;
    @apply shadow-lg;
    @apply rounded-md;
  }

  &--buttons {
    @apply relative;
    height: 300px;
    @apply mx-auto;
    width: Min(500px, 100%);

    a {
      @apply block;
      @apply absolute;

      img {
        width: 240px;
        @apply shadow-md;
        @apply rounded-xl;
      }

      &:nth-child(1) {
        @apply top-0;
      }

      &:nth-child(2) {
        top: 80px;
        @apply right-0;
      }

      &:nth-child(3) {
        top: 160px;
        left: 20px;
      }

      &:nth-child(4) {
        top: 240px;
        right: 0px;
      }
    }
  }
}

@screen md {
  .main-header {
    height: 735px;
  }

  #about-us {
    grid-template-columns: repeat(2, 1fr);

    &--desc {
      @apply my-auto;
      @apply mr-3;
    }
  }

  .endorsements {
    @apply gap-y-0;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(3, 1fr) auto;
  }

  .podcast {
    @apply gap-x-12;

    &--title {
      @apply row-start-1;
    }

    &--preview {
      @apply place-self-center;
      @apply row-start-1;
      @apply row-end-3;
    }

    &--buttons {
      @apply mx-0;
      @apply row-start-2;
      @apply col-start-1;
    }
  }
}

@screen lg {
  .card {
    &:nth-child(odd) {
      @apply place-self-start;
    }

    img {
      width: 200px;
      height: 200px;
    }

    &--details {
      @apply block;
      height: 200px;
    }

    &:hover {
      @apply relative;
      @apply z-10;

      .card--details {
        @apply opacity-100;
        transform: translateX(-5%);
      }
    }
  }
}
</style>
