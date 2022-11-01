export default {
  target: "static",
  head: {
    title: "SAFT Apologetics",
    meta: [
      { charset: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      { name: "msapplication-TileColor", content: "#da532c" },
      { name: "theme-color", content: "#ffffff" },
      { hid: "description", name: "description", content: "" },
    ],
    link: [
      { rel: "icon", type: "image/x-icon", href: "/favicon.ico" },
      { rel: "preconnect", href: "https://fonts.gstatic.com" },
      {
        rel: "stylesheet",
        href: "https://fonts.googleapis.com/css2?family=Arbutus+Slab&family=Montserrat:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap",
      },
      {
        rel: "apple-touch-icon",
        sizes: "180x180",
        href: "/apple-touch-icon.png",
      },
      { rel: "icon", type: "image/png", href: "/favicon-32x32.ico" },
      { rel: "icon", type: "image/png", href: "/favicon-16x16.ico" },
      { rel: "manifest", href: "/site.webmanifest" },
      {
        rel: "mask-icon",
        href: "/safari-pinned-tab.svg",
        color: "#000000",
      },
    ],
  },

  css: ["@/assets/css/main.scss"],

  plugins: [],

  components: true,

  buildModules: ["@nuxtjs/fontawesome"],

  modules: ["@nuxtjs/axios", "@nuxtjs/tailwindcss"],

  axios: {},

  fontawesome: {
    icons: {
      solid: true,
      brands: true,
      regular: true,
    },
  },

  build: {},
};
