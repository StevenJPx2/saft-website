<template>
  <div class="mt-16">
    <section class="container">
      <h1 class="mb-8" aos>SAFT Podcast</h1>
      <p aos>
        SAFT Podcast brings to you the rational and relevant defense of the
        Christian worldview. We also introduce to the audience the research done
        in the field of Christian Apologetics and the individuals who defend the
        Christian faith by featuring the greatest minds of the Christian
        Apologetics arena. This is a podcast for both the expert and the layman.
      </p>
      <br />
      <p aos>Listen to our latest episode here.</p>

      <div class="podcast">
        <article class="podcast--preview">
          <div id="buzzsprout-small-player-1034671" aos></div>
          <a href="https://saftpodcast.buzzsprout.com/" target="_blank" aos
            ><img src="@/assets/buzzsprout-icon.svg" /> Visit our podcast
            website</a
          >
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
      </div>
    </section>

    <section class="hero-dark">
      <div class="container flex flex-wrap">
        <article class="mb-16 infograph" aos>
          <p class="infograph--title">Our podcast is heard across...</p>
          <header class="infograph--info">
            <h3 class="infograph--info--title" id="country-no">0</h3>
            <p class="infograph--info--subtitle">countries</p>
          </header>
          <p class="infograph--sep">and</p>
          <header class="infograph--info">
            <h3 class="infograph--info--title" id="continent-no">0</h3>
            <p class="infograph--info--subtitle">continents</p>
          </header>
        </article>

        <article class="infograph" aos>
          <p class="infograph--title">We’re heard most in...</p>
          <header class="infograph--info">
            <h3 class="infograph--info--title">
              <img src="@/assets/us-white.svg" alt="US" />
            </h3>
            <p class="infograph--info--subtitle">US</p>
          </header>
          <p class="infograph--sep">followed by</p>
          <header class="infograph--info">
            <h3 class="infograph--info--title">
              <img src="@/assets/india-white.svg" alt="India" />
            </h3>
            <p class="infograph--info--subtitle">India</p>
          </header>
        </article>
      </div>
    </section>

    <section class="container">
      <h1 class="mt-16 mb-24" aos>Guests</h1>
      <div class="flex flex-wrap">
        <article v-for="(name, index) in guests" :key="index" class="guest" aos>
          <img
            :src="
              require(`@/assets/${name.toLowerCase().replace(/\s/g, '-')}.jpg`)
            "
            :alt="name"
          />
          <small>{{ name }}</small>
        </article>
      </div>
    </section>
  </div>
</template>

<script>
import { gsap } from "gsap";
import { ScrollTrigger } from "gsap/dist/ScrollTrigger";

export default {
  head() {
    return {
      title: "SAFT Apologetics | Podcast",
      script: [
        {
          type: "text/javascript",
          charset: "utf-8",
          src:
            "https://www.buzzsprout.com/1034671.js?container_id=buzzsprout-small-player-1034671&player=small&limit=1",
          body: true,
          defer: true,
        },
      ],
      meta: [
        {
          hid: "description",
          name: "description",
          content: `This is a podcast for the expert and the layman. 
          Join us in exploring the rational defense of Christianity.`,
        },
      ],
    };
  },

  mounted() {
    this.$store.commit("page", 2);

    gsap.registerPlugin(ScrollTrigger);

    let [countryNo, continentNo] = [35, 6];

    document.querySelectorAll("[aos]").forEach((el) => {
      gsap.fromTo(
        el,
        {
          opacity: 0,
          y: 50,
        },
        {
          opacity: 1,
          y: 0,
          duration: 1,
          ease: "power2.inOut",
          scrollTrigger: {
            trigger: el,
            toggleActions: "restart continue play reverse",
          },
        }
      );
    });

    document.querySelectorAll(".btn").forEach((el) => {
      gsap.fromTo(
        el,
        {
          opacity: 0,
          scale: 1.2,
        },
        {
          opacity: 1,
          scale: 1,
          duration: 2,
          ease: "elastic.out(1, 0.3)",
          scrollTrigger: {
            trigger: el,
            toggleActions: "restart continue play reverse",
          },
        }
      );
    });

    [
      { name: "#country-no", num: 35 },
      { name: "#continent-no", num: 6 },
    ].forEach((item) => {
      const animation = gsap.to(item.name, {
        duration: 2.5,
        ease: "power2.out",
      });

      const el = document.querySelector(item.name);

      animation.eventCallback("onUpdate", () => {
        el.innerHTML = Math.round(item.num * animation.ratio);
      });

      animation.seek(0);
      animation.pause(0);

      ScrollTrigger.create({
        trigger: item.name,
        start: "top center",
        end: "top top",
        onEnter() {
          if (el.innerHTML == 0) {
            animation.restart();
          }
        },
        onEnterBack() {
          if (!animation.isActive()) {
            el.innerHTML = 0;
            animation.seek(0);
            animation.pause(0);
          }
        },
        onLeaveBack() {
          if (el.innerHTML == 0) {
            animation.restart();
          }
        },
        onLeave() {
          if (!animation.isActive()) {
            el.innerHTML = 0;
            animation.seek(0);
            animation.pause(0);
          }
        },
      });
    });
  },

  methods: {},

  data() {
    return {
      guests: [
        "Brian Auten",
        "Greg Koukl",
        "Sean McDowell",
        "Justin Brierley",
        "William Lane Craig",
        "Frank Turek",
        "J Warner Wallace",
        "Michael L Brown",
        "Clay Jones",
        "Bo Bennett",
        "Abdu Murray",
        "Joel Ivy",
        "Godwin Thomas John",
        "Neil Reuben",
        "Jerin Thomas",
        "Chris BZ",
        "Sam Shamoun",
        "Lukas Rueegger",
        "Tom Gilson",
        "Alisa Childers",
      ],
    };
  },
};
</script>

<style lang="scss" scoped>
p {
  max-width: 800px;
}

.podcast {
  @apply grid;
  @apply py-16;
  @apply gap-y-8;

  &--preview {
    @apply w-full;
    div {
      @apply shadow-lg;
      @apply rounded-md;
    }

    a {
      @apply mt-8;
      @apply mx-auto;
      @apply flex;
      @apply items-center;
      @apply text-base;
      @apply w-max;
      background: #eceaea;
      @apply text-center;
      @apply py-4;
      @apply px-5;
      @apply rounded-lg;
      img {
        @apply mr-3;
        width: 35px;
      }
    }
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

.infograph {
  @apply text-white;
  @apply text-xl;
  @apply grid;
  @apply gap-8;
  @apply text-center;
  @apply flex-1;

  &--title {
    @apply place-self-center;
    @apply opacity-50;
  }

  &--info {
    &--title {
      @apply text-9xl;
      @apply leading-none;

      img {
        @apply mx-auto;
        max-width: 300px;
        @apply mb-3;
      }
    }
    &--subtitle {
      @apply opacity-60;
    }
  }

  &--sep {
    @apply place-self-center;
  }
}

.guest {
  @apply text-center;
  @apply mx-auto;
  @apply mb-16;

  img {
    max-width: 200px;
    @apply rounded-full;
  }
  small {
    @apply block;
    @apply mt-2;
    @apply font-medium;
    @apply text-sm;
  }
}

@screen md {
  .podcast {
    grid-template-columns: repeat(2, 1fr);
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

  .infograph {
    grid-template-columns: 1fr auto 1fr;

    &--title {
      @apply col-start-1;
      @apply col-end-4;
    }
  }

  .guest {
    @apply mr-10;
  }
}
</style>
