import { gsap } from "gsap";

export const state = () => ({
  pageNo: 0,
  directusUrl: "https://5ms1k56r.directus.app/",
});

export const mutations = {
  page(state, pageNo) {
    state.pageNo = pageNo;
  },

  aos(_, selectorString) {
    document.querySelectorAll(selectorString).forEach((el) => {
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
  },

  initAnimations() {
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

    document.querySelectorAll(".scroll-circle").forEach((el) => {
      gsap.fromTo(
        el,
        { y: 0 },
        {
          y: 30,
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
};
