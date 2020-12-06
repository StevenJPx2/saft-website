<template>
  <div class="container">
    <h1 class="main-header">
      Seeking
      <br />
      answers, <br />finding <br />truth.
    </h1>
    <div class="text-stuff is-hidden-mobile is-tablet"></div>
    <div id="head-oval" class="blue-oval"></div>
    <div id="big-body-oval" class="teal-oval is-hidden-mobile is-tablet"></div>
    <div
      id="small-body-oval"
      class="purple-oval is-hidden-mobile is-tablet"
    ></div>

    <section class="block pt-6">
      <div class="content block has-text-right">
        <h1 class="mb-1">WHO ARE WE?</h1>
        <p>
          We are a web-based ministry that aims to equip the believer defend
          their faith so that they may effectively evangelise.
        </p>
      </div>
      <div class="content block">
        <h1 class="mb-1">WHAT DO WE DO?</h1>
        <p>
          We live out our Mission and Vision by airing our bi-weekly SAFT
          Podcast and engage with live audience across languages via open
          forums.
        </p>
      </div>
    </section>
    <div id="big-body-oval" class="teal-oval is-hidden-tablet"></div>
    <h1 class="tagline">
      Equipping the believer defend their faith anytime, anywhere.
    </h1>
    <div id="small-body-oval" class="purple-oval is-hidden-tablet"></div>
    <h1 class="mt-6 mb-4 text-5xl font-black leading-none uppercase">
      Podcast Episodes
    </h1>

    <div
      id="buzzsprout-small-player-1034671-artist-SAFT-20Apologetics-limit-5"
    ></div>

    <script
      type="text/javascript"
      charset="utf-8"
      src="https://www.buzzsprout.com/1034671.js?player=small&artist=SAFT%20Apologetics&limit=3&container_id=buzzsprout-small-player-1034671-artist-SAFT-20Apologetics-limit-5"
    ></script>

    <b-button
      tag="a"
      href="https://saftpodcast.buzzsprout.com"
      target="_blank"
      class="all-epi-button"
      icon-right="arrow-right"
      type="is-light"
      >See all episodes
    </b-button>

    <section class="block">
      <h1 class="mt-6 mb-4 text-5xl font-black leading-none uppercase">
        Podcast Guests
      </h1>
      <div class="flex flex-wrap">
        <div class="" v-for="(guest, index) in guests" :key="index">
          <b-tooltip
            :label="guest.name"
            class=""
            :triggers="['hover', 'focus']"
          >
            <img
              class="is-64x64 rounded-full m-3"
              :src="guest.image"
              :alt="guest.name"
            />
          </b-tooltip>
        </div>
      </div>
    </section>

    <section class="podcast-badges">
      <a href="https://podcasts.apple.com/in/podcast/saft-podcast/id1511404295"
        ><img
          src="@/assets/Apple Podcasts.png"
          alt="Apple Podcasts"
          class="podcast-badge"
      /></a>

      <a
        href="https://podcasts.google.com/?feed=aHR0cHM6Ly9mZWVkcy5idXp6c3Byb3V0LmNvbS8xMDM0NjcxLnJzcw=="
        ><img
          src="@/assets/Google Podcasts.png"
          alt="Google Podcasts"
          class="ml-auto podcast-badge"
      /></a>

      <a href="https://open.spotify.com/show/4hOLouY5QFv3KuNNDUi5hM"
        ><img
          src="@/assets/Spotify Podcasts.png"
          alt="Spotify Podcasts"
          class="ml-4 podcast-badge"
      /></a>
    </section>
  </div>
</template>

<script>
export default {
  name: 'HomePage',
  head: {
    title: 'SAFT Apologetics',
  },
  data() {
    return {
      guests: [
        { name: 'Brian Auten', image: require('@/assets/brian-auten.jpg') },
        { name: 'Greg Koukl', image: require('@/assets/greg-koukl.jpg') },
        {
          name: 'Dr. Sean McDowell',
          image: require('@/assets/sean-mcdowell.jpg'),
        },
        {
          name: 'Justin Brierley',
          image: require('@/assets/justin-brierley.jpg'),
        },
        {
          name: 'Dr. William Lane Craig',
          image: require('@/assets/william-lane-craig.jpg'),
        },
        { name: 'Dr. Frank Turek', image: require('@/assets/frank-turek.jpg') },
        {
          name: 'J. Warner Wallace',
          image: require('@/assets/j-warner-wallace.jpg'),
        },
        {
          name: 'Dr. Michael Brown',
          image: require('@/assets/michael-brown.jpg'),
        },
        { name: 'Dr. Clay Jones', image: require('@/assets/clay-jones.jpg') },
        { name: 'Dr. Bo Bennett', image: require('@/assets/bo-bennett.jpg') },
        { name: 'Abdu Murray', image: require('@/assets/abdu-murray.jpg') },
        { name: 'Joel Ivy', image: require('@/assets/joel-ivy.jpg') },
        {
          name: 'Godwin Thomas John',
          image: require('@/assets/godwin-thomas.jpg'),
        },
        { name: 'Neil Reuben', image: require('@/assets/neil-reuben.jpg') },
        { name: 'Jerin Thomas', image: require('@/assets/jerin-thomas.jpg') },
        { name: 'Chris BZ', image: require('@/assets/chris-bz.jpg') },
        { name: 'Sam Shamoun', image: require('@/assets/sam-shamoun.jpg') },
      ],
    }
  },
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

  .content {
    h1 {
      @apply font-bold;
    }
  }

  #head-oval {
    @apply relative;
    left: 50%;
    top: -10%;
    width: Min(400px, 60vw);
    height: Min(400px, 60vw);
  }

  #big-body-oval {
    @apply relative;
    @apply mt-16;
    right: 20%;
    width: 220px;
    height: 220px;
  }

  .tagline {
    @apply pt-10;
    @apply pb-8;
    @apply text-center;
    @apply leading-relaxed;
    font-family: 'Arbutus Slab';
    font-size: Min(4vw, 2rem);
  }

  #small-body-oval {
    @apply relative;
    right: -80%;
    width: 80px;
    height: 80px;
  }

  .podcast-badges {
    max-width: 500px;
    @apply mx-auto;

    .podcast-badge {
      @apply my-12;
      width: 250px;
      @apply transition-all;
      @apply duration-300;
      @apply ease-in-out;
      &:hover,
      &:focus {
        @apply scale-125;
      }
    }
  }

  .all-epi-button {
    @apply mt-3;
    @apply flex;
  }

  img.is-64x64 {
    width: 64px;
    height: 64px;
  }
}

@screen md {
  .container {
    overflow-x: clip;
    @apply relative;

    .main-header {
      @apply mb-12;
    }

    .text-stuff {
      background-image: url('~assets/text-stuff.png');
      @apply absolute;
      top: -11%;
      height: 600px;
      width: 100%;
      opacity: 0.6;
      // box-shadow: 0px 10px 10px rgba(0, 0, 0, 0.1) inset;
    }

    #head-oval {
      @apply absolute;
      right: 0%;
      left: auto;
      top: 0%;
      width: Min(300px, 60vw);
      height: Min(300px, 60vw);
    }

    #big-body-oval {
      @apply absolute;
      @apply mt-16;
      right: 30%;
      top: 6%;
      width: 150px;
      height: 150px;
    }

    #small-body-oval {
      @apply absolute;
      right: 21%;
      top: 14%;
      width: 80px;
      height: 80px;
    }

    .tagline {
      @apply py-24;
      @apply text-center;
      @apply leading-relaxed;
      font-family: 'Arbutus Slab';
      font-size: Min(4vw, 2rem);
    }
  }
}

@screen lg {
  .container {
    overflow-x: initial;

    #big-body-oval {
      @apply absolute;
      @apply mt-16;
      right: 26%;
      top: 4%;
      width: 150px;
      height: 150px;
    }
  }
}
</style>
