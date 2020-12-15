<template>
  <div>
    <header class="hero main-header">
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
    </header>

    <section class="container-light">
      <div id="about-us" class="container relative">
        <div id="founder-images">
          <div class="card">
            <img src="@/assets/jacob-varghese.jpg" alt="Jacob Varghese" />
            <div class="card--details">
              <h2 class="card--details--title">Jacob Varghese</h2>
              <h3 class="card--details--subtitle">Founder - Director</h3>
              <p class="card--details--desc">
                Born to parents hailing from Kerala, who to the dismay and
                resistance of his grandparents decided to follow God and know
                Him...
              </p>
            </div>
          </div>

          <div class="card">
            <img src="@/assets/ankit-nayak.jpg" alt="Ankit Nayak" />
            <div class="card--details">
              <h2 class="card--details--title">Ankit Nayak</h2>
              <h3 class="card--details--subtitle">Executive Director</h3>
              <p class="card--details--desc">
                Ankit has been an active part of Assembly of God Church
                ministries in Kolkata and India Campus Crusade for Christ.
                Passionate about...
              </p>
            </div>
          </div>

          <div class="card">
            <img src="@/assets/steven-john.jpg" alt="Steven John" />
            <div class="card--details">
              <h2 class="card--details--title">Steven John</h2>
              <h3 class="card--details--subtitle">Chief Editor</h3>
              <p class="card--details--desc">
                Steven John has always been an active and passionate evangelism
                oriented student with a deep seated desire to further his innate
                talent in design and...
              </p>
            </div>
          </div>
        </div>

        <div id="about-us--desc">
          <h2 class="uppercase">Who are we?</h2>
          <p>
            We are a web-based ministry that aims to equip the believer defend
            their faith so that they may effectively evangelise.
          </p>
          <br />
          <h2 class="uppercase">What do we do?</h2>
          <p>
            We live out our Mission and Vision by airing our bi-weekly SAFT
            Podcast and engaging with live audience across languages via open
            forums.
          </p>
          <div class="flex w-full mt-4">
            <div class="w-full"></div>
            <a href="#" class="flex-none btn"
              >More<icon class="ml-2" name="arrow-right" />
            </a>
          </div>
        </div>
      </div>
    </section>

    <section class="container relative py-24">
      <div id="tagline" class="flex items-center">
        <blockquote class="z-10 my-auto">
          Equipping the believer defend their faith anytime, anywhere.
        </blockquote>
      </div>
    </section>
    <section class="container-light">
      <div class="container endorsements">
        <section class="endorsements--card">
          <p class="endorsements--card--desc">
            They are building a great resource and a much-needed ministry...
          </p>
          <h3 class="endorsements--card--name">Brian Auten</h3>
          <h4 class="endorsements--card--title">
            Founder-Emeritus, Apologetics315
          </h4>
        </section>
        <section class="endorsements--card md:row-start-2 md:col-start-2">
          <p class="endorsements--card--desc">
            ...My Best Wishes, Prayers and Blessings goes with the SAFT team...
          </p>
          <h3 class="endorsements--card--name">P Anupam Nayak</h3>
          <h4 class="endorsements--card--title">
            Associate National Director, India Campus Crusade for Christ, East
            India
          </h4>
        </section>
        <section class="endorsements--card md:row-start-3 md:col-start-1">
          <p class="endorsements--card--desc">
            ...I highly commend them to you...
          </p>
          <h3 class="endorsements--card--name">Jerin Thomas</h3>
          <h4 class="endorsements--card--title">
            Global Mission Catalyst, WCD of the Christian and Missionary
            Alliance, Canada
          </h4>
        </section>
        <a
          href="#"
          class="mt-4 btn md:col-start-2 md:row-start-4 place-self-end"
          >More<icon class="ml-2" name="arrow-right" />
        </a>
      </div>
    </section>
    <section class="container podcast">
      <article class="podcast--title">
        <h2 class="mb-3 text-5xl uppercase">Saft Podcast</h2>
        <p class="text-xl">Listen to our latest episode here.</p>
      </article>

      <article class="podcast--preview">
        <div id="buzzsprout-small-player-1034671"></div>
      </article>

      <article class="podcast--buttons">
        <a
          href="https://podcasts.apple.com/in/podcast/saft-podcast/id1511404295"
          target="_blank"
          ><img
            src="@/assets/apple-podcasts.png"
            alt="Listen on Apple Podcasts"
        /></a>
        <a
          href="https://podcasts.google.com/?feed=aHR0cHM6Ly9mZWVkcy5idXp6c3Byb3V0LmNvbS8xMDM0NjcxLnJzcw=="
          target="_blank"
          ><img
            src="@/assets/google-podcasts.png"
            alt="Listen on Google Podcasts"
        /></a>
        <a
          href="https://open.spotify.com/show/4hOLouY5QFv3KuNNDUi5hM"
          target="_blank"
          ><img src="@/assets/spotify-podcasts.png" alt="Listen on Spotify"
        /></a>
        <a
          href="https://www.youtube.com/channel/UCBDroMQT6UM9RCK3vjdW6dA/videos"
          target="_blank"
          ><img src="@/assets/youtube-podcasts.png" alt="Watch on Youtube"
        /></a>
      </article>
      <a href="#" class="mt-4 btn md:col-start-2 md:row-start-4 place-self-end"
        >More<icon class="ml-2" name="arrow-right" />
      </a>
    </section>
  </div>
</template>

<script>
export default {
  head() {
    return {
      title: "SAFT Apologetics",
      script: [
        {
          type: "text/javascript",
          charset: "utf-8",
          src:
            "https://www.buzzsprout.com/1034671.js?container_id=buzzsprout-small-player-1034671&player=small&limit=1",
          body: true,
        },
      ],
    };
  },

  created() {
    this.$store.commit("page", 0);
    this.$router;
  },
};
</script>

<style lang="scss" scoped>
.main-header {
  @apply flex;
  @apply relative;
  background-color: #001f6d;
  height: 87vh;
  @apply items-center;
  @apply overflow-x-hidden;

  h1 {
    font-size: Min(3.75rem, 15vw);
  }

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
  @apply shadow-lg;
  @apply w-max;
  height: max-content;
  @apply rounded-xl;
  @apply bg-white;
  @apply grid;

  img {
    @apply rounded-xl;
    width: 120px;
    @apply row-span-3;
  }

  &--details {
    @apply hidden;
    @apply px-5;
    @apply py-5;

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

  &--title {
  }

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

    #main-circle {
      left: 450px;
      top: 90px;
    }

    #main-circle-2 {
      left: 360px;
      top: 440px;
    }
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
  .main-header {
    #main-circle {
      left: 770px;
    }

    #main-circle-2 {
      left: 670px;
    }
  }

  .card {
    &:nth-child(odd) {
      @apply place-self-start;
    }

    img {
      width: 200px;
    }

    &:hover {
      @apply relative;
      @apply z-10;
      grid-template-columns: max-content 300px;

      .card--details {
        @apply block;
      }
    }
  }
}

@screen 2xl {
  .main-header {
    #main-circle {
      left: 1120px;
    }

    #main-circle-2 {
      left: 1020px;
    }
  }
}
</style>
