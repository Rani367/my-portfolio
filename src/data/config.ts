const config = {
  title: "Rani | Developer",
  description: {
    long: "Hey! I'm Rani, a high school student passionate about coding. I'm self-teaching web development with Next.js and React while learning Python, C, and C# at school. Check out my projects like Hativon (my school's newspaper site) and my open source contributions to Zed editor!",
    short:
      "High school student and self-taught developer building web apps and contributing to open source.",
  },
  keywords: [
    "Rani",
    "portfolio",
    "developer",
    "high school",
    "web development",
    "Next.js",
    "React",
    "TypeScript",
    "Python",
    "C",
    "open source",
    "Hativon",
    "Zed",
    "Neovim",
  ],
  author: "Rani",
  email: "rani2011367@gmail.com",
  site: "https://rani367.github.io",

  // for github stars button
  githubUsername: "Rani367",
  githubRepo: "3d-portfolio",

  get ogImg() {
    return this.site + "/assets/seo/og-image.png";
  },
  social: {
    github: "https://github.com/Rani367",
  },
};
export { config };
