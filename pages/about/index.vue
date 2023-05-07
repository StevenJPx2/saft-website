<template>
  <div>
    <section class="container pt-12 about-us">
      <h1 class="mb-8" aos>About Us</h1>
      <article
        class="about-us--desc"
        aos
        v-html="aboutPageData.about_us"
      ></article>
    </section>

    <section class="flex">
      <div class="ml-auto scroll-circle">
        <div
          class="relative w-48 h-48 overflow-x-hidden rounded-full shadow-2xl md:h-72 md:w-72 light-gradient"
        ></div>
        <div
          class="relative w-16 h-16 rounded-full shadow-2xl -top-10 -left-1/4 md:h-28 md:w-28 middle-gradient"
        ></div>
      </div>
    </section>

    <section class="mt-0 from-founder container-light">
      <article class="text-article">
        <h1 class="mb-8" aos>From Founder's Desk</h1>
        <div
          class="from-founder--text [&>p]:mb-3"
          aos
          v-html="aboutPageData.from_founders_desk"
        ></div>
      </article>
    </section>

    <section class="container-dark">
      <article class="container mission-vision">
        <div class="mission-vision--text">
          <h2 aos>MISSION</h2>
          <p aos>
            {{ aboutPageData.mission }}
          </p>
        </div>
        <div class="mission-vision--text">
          <h2 aos>VISION</h2>
          <p aos>
            {{ aboutPageData.vision }}
          </p>
        </div>
      </article>
    </section>

    <section class="pt-10 overflow-hidden pb-14">
      <div class="absolute w-full">
        <div
          class="relative w-48 h-48 rounded-full shadow-2xl -left-20 md:h-72 md:w-72 dark-gradient scroll-circle"
        ></div>
      </div>
    </section>

    <section class="h-48 md:h-80"></section>

    <section id="core-team" class="container pt-6 md:pt-0 core-team">
      <h1 class="mb-24 md:mb-32" aos>Our Core Team</h1>
      <article
        class="core-team--card"
        v-for="{ id, name, title, description, img } in coreTeamMembers"
        :key="id"
      >
        <img
          :src="`https://5ms1k56r.directus.app/assets/${img}`"
          :alt="name"
          aos
        />
        <h3 aos>{{ name }}</h3>
        <h4 aos>{{ title }}</h4>
        <p aos>{{ description }}</p>
      </article>
    </section>

    <section class="container-light">
      <div class="container">
        <h1 class="mb-8" aos>Advisory Board</h1>
        <p aos>{{ aboutPageData.advisory_board_description }}</p>

        <article class="advisory-board">
          <section
            class="advisory-board--card"
            v-for="{ id, name, title, img } in advisoryBoardMembers"
            :key="id"
            aos
          >
            <img
              :src="`https://5ms1k56r.directus.app/assets/${img}`"
              :alt="name"
              aos
            />
            <p>{{ name }}</p>
            <small>{{ title }}</small>
          </section>
        </article>
      </div>
    </section>

    <section class="w-full pt-10 pb-14">
      <div
        class="relative w-48 h-48 ml-auto rounded-full shadow-2xl md:h-72 md:w-72 darker-gradient scroll-circle"
      ></div>
    </section>

    <section id="endorsements" class="container endorsements">
      <h1 class="mb-12 md:mb-24" aos>Endorsements</h1>
      <article
        class="endorsements--card"
        v-for="{ id, name, title, body } in endorsements"
        :key="id"
      >
        <div v-html="body" class="mb-3"></div>
        <h5>{{ name }}</h5>
        <p>{{ title }}</p>
      </article>
    </section>

    <section class="container-light">
      <article class="statement text-article" aos>
        <h1 class="mb-8">Statement of Faith</h1>
        <div class="[&>p]:mb-3" v-html="aboutPageData.statement_of_faith"></div>
      </article>
    </section>
  </div>
</template>

<script lang="ts">
import { gsap } from "gsap";
import { ScrollTrigger } from "gsap/dist/ScrollTrigger";
import { defineComponent } from "vue";
import { groq } from "@nuxtjs/sanity";

export default defineComponent({
  head() {
    return {
      title: "SAFT Apologetics | About Us",
      meta: [
        {
          hid: "description",
          name: "description",
          content: `We seek to stir conversations about the most important
          questions of mankind.`,
        },
      ],
    };
  },

  mounted() {
    this.$store.commit("page", 1);

    gsap.registerPlugin(ScrollTrigger);

    this.$store.commit("initAnimations");
    this.$store.commit("aos", ".endorsements--card");

    document.querySelectorAll(".scroll-circle").forEach((el) => {
      gsap.fromTo(
        el,
        { y: -10 },
        {
          y: 60,
          ease: "sine.out",
          scrollTrigger: {
            trigger: el,
            scrub: true,
            start: "top center",
            end: "bottom center",
          },
        }
      );
    });
  },

  async asyncData({ app: { $sanity } }) {
    const aboutPageData = await $sanity.fetch(groq`*[_type == 'aboutPage'][0] {
  fromFoundersDesk,
  vision,
  mission,
  aboutUs,
  statementOfFaith,
  advisoryBoardDescription
}`);
    const coreTeamMembers = await $sanity.fetch<
      {
        _id: string;
        title: string;
        name: string;
        description: string;
        imageId?: string;
      }[]
    >(groq`*[_type == 'coreTeam'] {
  _id,
  title,
  name,
  description,
  "imageId": img.asset._ref
}`);
    const endorsements = await $sanity.fetch(groq`*[_type == 'endorsements'] {
  _id,
  name,
  body,
  title
}`);
    const advisoryBoardMembers =
      await $sanity.fetch(groq`*[_type == 'advisoryBoard'] {
  name,
  title
}`);

    return {
      aboutPageData,
      coreTeamMembers,
      endorsements,
      advisoryBoardMembers,
    };
  },
});
</script>

<style lang="scss" scoped>
.from-founder {
  &--text {
    p {
      @apply mb-5;
    }
  }
}

.mission-vision {
  @apply grid;
  @apply text-white;
  @apply gap-7;

  &--text {
    h2 {
      @apply mb-2;
    }
  }
}

.core-team {
  &--card {
    @apply grid;
    @apply mb-24;

    img {
      @apply rounded-xl;
      @apply mb-6;
      max-width: 300px;
    }

    h3 {
      @apply text-3xl;
      @apply font-semibold;
    }

    h4 {
      @apply text-xl;
      color: #6c6c6c;
    }

    p {
      @apply mt-3;
      max-width: 600px;
    }
  }
}

.advisory-board {
  @apply flex;
  @apply flex-wrap;
  @apply mt-12;

  &--card {
    @apply flex-1;
    @apply text-center;
    @apply mb-12;
    @apply px-3;

    img {
      width: 210px;
      @apply max-w-none;
      @apply mx-auto;
      @apply rounded-lg;
    }

    p {
      @apply mt-3;
      @apply font-medium;
    }

    small {
      @apply block;
      color: #717171;
    }
  }
}

.endorsements {
  @apply mb-24;

  &--card {
    @apply mb-12;
    max-width: 800px;

    p {
      @apply mb-3;
    }

    h5 {
      @apply uppercase;
      @apply font-bold;
      @apply text-lg;
    }
  }
}

.statement {
  p {
    @apply mb-4;
  }

  small {
    @apply mt-3;
    @apply block;
    color: #717171;
  }
}

@screen md {
  .about-us {
    &--desc {
      columns: 500px;

      p {
        -webkit-column-break-inside: avoid;
        page-break-inside: avoid;
        break-inside: avoid;
      }
    }
  }

  .mission-vision {
    grid-template-columns: repeat(2, 1fr);
  }

  .core-team {
    &--card {
      grid-template-rows: auto auto 1fr;
      @apply mb-24;

      img {
        @apply row-start-1;
        @apply row-end-4;
        @apply mr-10;
      }

      h3 {
        @apply col-start-2;
      }

      h4 {
        @apply col-start-2;
      }

      p {
        @apply col-start-2;
      }
    }
  }
}
</style>
