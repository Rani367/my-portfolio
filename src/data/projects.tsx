import AceTernityLogo from "@/components/logos/aceternity";
import SlideShow from "@/components/slide-show";
import { Button } from "@/components/ui/button";
import { TypographyH3, TypographyP } from "@/components/ui/typography";
import { ArrowUpRight, ExternalLink, Link2, MoveUpRight } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { ReactNode } from "react";
import { RiNextjsFill, RiNodejsFill, RiReactjsFill } from "react-icons/ri";
import {
  SiChakraui,
  SiDocker,
  SiExpress,
  SiFirebase,
  SiJavascript,
  SiMongodb,
  SiPostgresql,
  SiPrisma,
  SiPython,
  SiReactquery,
  SiRust,
  SiSanity,
  SiShadcnui,
  SiSocketdotio,
  SiSupabase,
  SiTailwindcss,
  SiThreedotjs,
  SiTypescript,
  SiVuedotjs,
  SiNeovim,
  SiLua,
  SiC,
} from "react-icons/si";
import { TbBrandFramerMotion } from "react-icons/tb";
const BASE_PATH = "/assets/projects-screenshots";

const ProjectsLinks = ({ live, repo }: { live: string; repo?: string }) => {
  return (
    <div className="flex flex-col md:flex-row items-center justify-start gap-3 my-3 mb-8">
      <Link
        className="font-mono underline flex gap-2"
        rel="noopener"
        target="_new"
        href={live}
      >
        <Button variant={"default"} size={"sm"}>
          Visit Website
          <ArrowUpRight className="ml-3 w-5 h-5" />
        </Button>
      </Link>
      {repo && (
        <Link
          className="font-mono underline flex gap-2"
          rel="noopener"
          target="_new"
          href={repo}
        >
          <Button variant={"default"} size={"sm"}>
            Github
            <ArrowUpRight className="ml-3 w-5 h-5" />
          </Button>
        </Link>
      )}
    </div>
  );
};

export type Skill = {
  title: string;
  bg: string;
  fg: string;
  icon: ReactNode;
};
const PROJECT_SKILLS = {
  next: {
    title: "Next.js",
    bg: "black",
    fg: "white",
    icon: <RiNextjsFill />,
  },
  chakra: {
    title: "Chakra UI",
    bg: "black",
    fg: "white",
    icon: <SiChakraui />,
  },
  node: {
    title: "Node.js",
    bg: "black",
    fg: "white",
    icon: <RiNodejsFill />,
  },
  python: {
    title: "Python",
    bg: "black",
    fg: "white",
    icon: <SiPython />,
  },
  prisma: {
    title: "prisma",
    bg: "black",
    fg: "white",
    icon: <SiPrisma />,
  },
  postgres: {
    title: "PostgreSQL",
    bg: "black",
    fg: "white",
    icon: <SiPostgresql />,
  },
  mongo: {
    title: "MongoDB",
    bg: "black",
    fg: "white",
    icon: <SiMongodb />,
  },
  express: {
    title: "Express",
    bg: "black",
    fg: "white",
    icon: <SiExpress />,
  },
  reactQuery: {
    title: "React Query",
    bg: "black",
    fg: "white",
    icon: <SiReactquery />,
  },
  shadcn: {
    title: "ShanCN UI",
    bg: "black",
    fg: "white",
    icon: <SiShadcnui />,
  },
  aceternity: {
    title: "Aceternity",
    bg: "black",
    fg: "white",
    icon: <AceTernityLogo />,
  },
  tailwind: {
    title: "Tailwind",
    bg: "black",
    fg: "white",
    icon: <SiTailwindcss />,
  },
  docker: {
    title: "Docker",
    bg: "black",
    fg: "white",
    icon: <SiDocker />,
  },
  yjs: {
    title: "Y.js",
    bg: "black",
    fg: "white",
    icon: (
      <span>
        <strong>Y</strong>js
      </span>
    ),
  },
  firebase: {
    title: "Firebase",
    bg: "black",
    fg: "white",
    icon: <SiFirebase />,
  },
  sockerio: {
    title: "Socket.io",
    bg: "black",
    fg: "white",
    icon: <SiSocketdotio />,
  },
  js: {
    title: "JavaScript",
    bg: "black",
    fg: "white",
    icon: <SiJavascript />,
  },
  ts: {
    title: "TypeScript",
    bg: "black",
    fg: "white",
    icon: <SiTypescript />,
  },
  vue: {
    title: "Vue.js",
    bg: "black",
    fg: "white",
    icon: <SiVuedotjs />,
  },
  react: {
    title: "React.js",
    bg: "black",
    fg: "white",
    icon: <RiReactjsFill />,
  },
  sanity: {
    title: "Sanity",
    bg: "black",
    fg: "white",
    icon: <SiSanity />,
  },
  spline: {
    title: "Spline",
    bg: "black",
    fg: "white",
    icon: <SiThreedotjs />,
  },
  gsap: {
    title: "GSAP",
    bg: "black",
    fg: "white",
    icon: "",
  },
  framerMotion: {
    title: "Framer Motion",
    bg: "black",
    fg: "white",
    icon: <TbBrandFramerMotion />,
  },
  supabase: {
    title: "Supabase",
    bg: "black",
    fg: "white",
    icon: <SiSupabase />,
  },
  rust: {
    title: "Rust",
    bg: "black",
    fg: "white",
    icon: <SiRust />,
  },
  neovim: {
    title: "Neovim",
    bg: "black",
    fg: "white",
    icon: <SiNeovim />,
  },
  lua: {
    title: "Lua",
    bg: "black",
    fg: "white",
    icon: <SiLua />,
  },
  c: {
    title: "C",
    bg: "black",
    fg: "white",
    icon: <SiC />,
  },
};
export type Project = {
  id: string;
  category: string;
  title: string;
  src: string;
  screenshots: string[];
  skills: { frontend: Skill[]; backend: Skill[] };
  content: React.ReactNode | any;
  github?: string;
  live: string;
};
const projects: Project[] = [
  {
    id: "hativon",
    category: "School Newspaper",
    title: "Hativon",
    src: "/assets/projects-screenshots/portfolio/landing.png",
    screenshots: [],
    skills: {
      frontend: [
        PROJECT_SKILLS.ts,
        PROJECT_SKILLS.next,
        PROJECT_SKILLS.tailwind,
      ],
      backend: [
        PROJECT_SKILLS.postgres,
      ],
    },
    live: "https://github.com/Rani367/Hativon",
    github: "https://github.com/Rani367/Hativon",
    get content() {
      return (
        <div>
          <TypographyP className="font-mono text-2xl text-center">
            My School&apos;s Newspaper Website
          </TypographyP>
          <TypographyP className="font-mono">
            Hativon is a newspaper website that I built for my school. It&apos;s
            actively used by students and teachers to read and publish articles.
            Built with Next.js, TypeScript, and PostgreSQL, it showcases my
            ability to create real-world applications that serve actual users.
          </TypographyP>
          <ProjectsLinks live={this.live} repo={this.github} />
          <TypographyH3 className="my-4 mt-8">Tech Stack</TypographyH3>
          <ul className="list-disc ml-6 font-mono">
            <li>Next.js for the frontend framework</li>
            <li>TypeScript for type-safe code</li>
            <li>PostgreSQL for the database</li>
            <li>Tailwind CSS for styling</li>
          </ul>
          <TypographyH3 className="my-4 mt-8">Impact</TypographyH3>
          <p className="font-mono mb-2">
            This project taught me a lot about building production applications
            that real people use. It&apos;s one thing to build a side project,
            but seeing my classmates actually using something I built is amazing.
          </p>
        </div>
      );
    },
  },
  {
    id: "zed-contribution",
    category: "Open Source",
    title: "Zed Editor Contribution",
    src: "/assets/projects-screenshots/portfolio/landing.png",
    screenshots: [],
    skills: {
      frontend: [],
      backend: [PROJECT_SKILLS.rust],
    },
    live: "https://github.com/zed-industries/zed/pull/43872",
    github: "https://github.com/zed-industries/zed/pull/43872",
    get content() {
      return (
        <div>
          <TypographyP className="font-mono text-2xl text-center">
            Contributing to Zed Editor
          </TypographyP>
          <TypographyP className="font-mono">
            I contributed to Zed, a high-performance code editor built in Rust.
            My PR added upsell banners for integrated extensions in the
            extensions UI, and it got merged!
          </TypographyP>
          <ProjectsLinks live={this.live} repo={this.github} />
          <TypographyH3 className="my-4 mt-8">The Contribution</TypographyH3>
          <p className="font-mono mb-2">
            PR #43872: &quot;extensions_ui: Add upsell banners for integrated extensions&quot;
          </p>
          <p className="font-mono mb-2">
            This was my first contribution to a major open source project, and
            it taught me a lot about working with large codebases and
            collaborating with other developers.
          </p>
          <TypographyH3 className="my-4 mt-8">What I Learned</TypographyH3>
          <ul className="list-disc ml-6 font-mono">
            <li>Working with Rust in a real-world codebase</li>
            <li>Open source collaboration workflows</li>
            <li>Code review processes</li>
            <li>Reading and understanding existing code</li>
          </ul>
        </div>
      );
    },
  },
  {
    id: "satellite",
    category: "Systems Programming",
    title: "Satellite Program",
    src: "/assets/projects-screenshots/portfolio/landing.png",
    screenshots: [],
    skills: {
      frontend: [],
      backend: [PROJECT_SKILLS.c],
    },
    live: "#",
    get content() {
      return (
        <div>
          <TypographyP className="font-mono text-2xl text-center">
            C Programming Project
          </TypographyP>
          <TypographyP className="font-mono">
            A satellite-related programming project built in C. This project
            helped me dive deep into systems programming and understand how
            low-level code works.
          </TypographyP>
          <TypographyH3 className="my-4 mt-8">About This Project</TypographyH3>
          <p className="font-mono mb-2">
            This is a private repository project that I&apos;m working on as part
            of my learning journey with C programming. It involves concepts like
            memory management, pointers, and systems-level programming.
          </p>
          <TypographyH3 className="my-4 mt-8">Skills Developed</TypographyH3>
          <ul className="list-disc ml-6 font-mono">
            <li>C programming fundamentals</li>
            <li>Memory management and pointers</li>
            <li>Systems programming concepts</li>
            <li>Low-level problem solving</li>
          </ul>
        </div>
      );
    },
  },
  {
    id: "dotfiles",
    category: "Developer Tools",
    title: "Neovim Config",
    src: "/assets/projects-screenshots/portfolio/landing.png",
    screenshots: [],
    skills: {
      frontend: [],
      backend: [PROJECT_SKILLS.neovim, PROJECT_SKILLS.lua],
    },
    live: "https://github.com/Rani367/dotfiles",
    github: "https://github.com/Rani367/dotfiles",
    get content() {
      return (
        <div>
          <TypographyP className="font-mono text-2xl text-center">
            My Neovim Configuration
          </TypographyP>
          <TypographyP className="font-mono">
            A modern Neovim configuration for version 0.11+. I use Neovim as my
            daily driver for coding, and I&apos;ve customized it to fit my workflow
            perfectly.
          </TypographyP>
          <ProjectsLinks live={this.live} repo={this.github} />
          <TypographyH3 className="my-4 mt-8">Why Neovim?</TypographyH3>
          <p className="font-mono mb-2">
            I love the efficiency of modal editing and the endless customization
            possibilities. My config includes LSP support, treesitter for syntax
            highlighting, and a bunch of plugins that make coding a joy.
          </p>
          <TypographyH3 className="my-4 mt-8">Features</TypographyH3>
          <ul className="list-disc ml-6 font-mono">
            <li>Modern Neovim 0.11+ features</li>
            <li>Lua-based configuration</li>
            <li>LSP integration for multiple languages</li>
            <li>Custom keybindings optimized for my workflow</li>
          </ul>
          <p className="font-mono mb-2 mt-8 text-center">
            There&apos;s something special about crafting your own dev environment.
          </p>
        </div>
      );
    },
  },
];
export default projects;
