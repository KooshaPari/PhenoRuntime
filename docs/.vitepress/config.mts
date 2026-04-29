import { defineConfig } from "vitepress";

export default defineConfig({
  title: "PhenoRuntime",
  description: "Runtime substrate and adapter layer for the Phenotype ecosystem",
  base: process.env.GITHUB_PAGES === "true" ? "/PhenoRuntime/" : "/",
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: "Overview", link: "/" },
      { text: "Runtime", link: "/phenoruntime/" },
      { text: "ADRs", link: "/adr/" },
      { text: "Research", link: "/research/" },
      { text: "Worklogs", link: "/worklogs/" },
    ],
    sidebar: [
      {
        text: "Getting Started",
        items: [
          { text: "Overview", link: "/" },
          { text: "Runtime surface", link: "/phenoruntime/" },
        ],
      },
      {
        text: "Architecture",
        items: [
          { text: "ADRs", link: "/adr/" },
          { text: "Research", link: "/research/" },
          { text: "Worklogs", link: "/worklogs/" },
        ],
      },
    ],
    socialLinks: [
      { icon: "github", link: "https://github.com/KooshaPari/PhenoRuntime" },
    ],
  },
});
