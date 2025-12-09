// ============================================
// Configuration Data
// ============================================

const dockApps = [
  { id: "finder", name: "Portfolio", icon: "finder.png", canOpen: true },
  { id: "safari", name: "Safari", icon: "safari.png", canOpen: true },
  { id: "photos", name: "Gallery", icon: "photos.png", canOpen: true },
  { id: "contact", name: "Contact", icon: "contact.png", canOpen: true },
  { id: "terminal", name: "Skills", icon: "terminal.png", canOpen: true },
  { id: "trash", name: "Archive", icon: "trash.png", canOpen: false },
];

const techStack = [
  { category: "Frontend", items: ["React.js", "Next.js", "TypeScript"] },
  { category: "Mobile", items: ["React Native", "Expo"] },
  { category: "Styling", items: ["Tailwind CSS", "Sass", "CSS"] },
  { category: "Backend", items: ["Node.js", "Express", "NestJS", "Hono"] },
  { category: "Database", items: ["MongoDB", "PostgreSQL"] },
  { category: "Dev Tools", items: ["Git", "GitHub", "Docker"] },
];

const socials = [
  {
    id: 1,
    text: "Github",
    icon: "/public/icons/github.svg",
    bg: "#f4656b",
    link: "https://github.com/JavaScript-Mastery-Pro",
  },
  {
    id: 2,
    text: "Platform",
    icon: "/public/icons/atom.svg",
    bg: "#4bcb63",
    link: "https://jsmastery.com/",
  },
  {
    id: 3,
    text: "Twitter/X",
    icon: "/public/icons/twitter.svg",
    bg: "#ff866b",
    link: "https://x.com/jsmasterypro",
  },
  {
    id: 4,
    text: "LinkedIn",
    icon: "/public/icons/linkedin.svg",
    bg: "#05b6f6",
    link: "https://www.linkedin.com/company/javascriptmastery/posts/?feedView=all",
  },
];

const photosLinks = [
  { id: 1, icon: "/public/icons/gicon1.svg", title: "Library" },
  { id: 2, icon: "/public/icons/gicon2.svg", title: "Memories" },
  { id: 3, icon: "/public/icons/file.svg", title: "Places" },
  { id: 4, icon: "/public/icons/gicon4.svg", title: "People" },
  { id: 5, icon: "/public/icons/gicon5.svg", title: "Favorites" },
];

const gallery = [
  { id: 1, img: "/public/images/gal1.png" },
  { id: 2, img: "/public/images/gal2.png" },
  { id: 3, img: "/public/images/gal3.png" },
  { id: 4, img: "/public/images/gal4.png" },
];

const locations = {
  work: {
    id: 1,
    type: "work",
    name: "Work",
    icon: "/public/icons/work.svg",
    kind: "folder",
    children: [
      {
        id: 5,
        name: "Nike Ecommerce Website Application",
        icon: "/public/images/folder.png",
        kind: "folder",
        position: "top-10 left-5",
        children: [
          {
            id: 1,
            name: "Nike Project.txt",
            icon: "/public/images/txt.png",
            kind: "file",
            fileType: "txt",
            position: "top-5 left-10",
            description: [
              "The Nike eCommerce website is a sleek and modern platform designed for shopping the latest Nike collections.",
              "Instead of a simple online store, it delivers an immersive experience with bold visuals, interactive product displays, and smooth navigation.",
              "Think of it like walking into a flagship Nike store—but right from your phone or laptop.",
              "It's built with Next.js and Tailwind, ensuring fast performance, responsive design, and a clean, premium look.",
            ],
          },
          {
            id: 2,
            name: "nike.com",
            icon: "/public/images/safari.png",
            kind: "file",
            fileType: "url",
            href: "https://youtu.be/fZdTYswuZjU?si=Awjl-pIst9e09_UU",
            position: "top-10 right-20",
          },
          {
            id: 4,
            name: "nike.png",
            icon: "/public/images/image.png",
            kind: "file",
            fileType: "img",
            position: "top-52 right-80",
            imageUrl: "/public/images/project-1.png",
          },
          {
            id: 5,
            name: "Design.fig",
            icon: "/public/images/plain.png",
            kind: "file",
            fileType: "fig",
            href: "https://google.com",
            position: "top-60 right-20",
          },
        ],
      },
      {
        id: 6,
        name: "AI Resume Analyzer",
        icon: "/public/images/folder.png",
        kind: "folder",
        position: "top-52 right-80",
        children: [
          {
            id: 1,
            name: "AI Resume Analyzer Project.txt",
            icon: "/public/images/txt.png",
            kind: "file",
            fileType: "txt",
            position: "top-5 right-10",
            description: [
              "AI Resume Analyzer is a smart tool that helps you perfect your resume with instant feedback.",
              "Instead of guessing what recruiters want, you get AI-powered insights on keywords, formatting, and overall impact.",
              "Think of it like having a career coach—pointing out strengths, fixing weaknesses, and boosting your chances of landing interviews.",
              "It's built with Next.js and Tailwind, so it runs fast, looks professional, and works seamlessly on any device.",
            ],
          },
          {
            id: 2,
            name: "ai-resume-analyzer.com",
            icon: "/public/images/safari.png",
            kind: "file",
            fileType: "url",
            href: "https://youtu.be/iYOz165wGkQ?si=R1hs8Legl200m0Cl",
            position: "top-20 left-20",
          },
          {
            id: 4,
            name: "ai-resume-analyzer.png",
            icon: "/public/images/image.png",
            kind: "file",
            fileType: "img",
            position: "top-52 left-80",
            imageUrl: "/public/images/project-2.png",
          },
          {
            id: 5,
            name: "Design.fig",
            icon: "/public/images/plain.png",
            kind: "file",
            fileType: "fig",
            href: "https://google.com",
            position: "top-60 left-5",
          },
        ],
      },
      {
        id: 7,
        name: "Food Delivery App",
        icon: "/public/images/folder.png",
        kind: "folder",
        position: "top-10 left-80",
        children: [
          {
            id: 1,
            name: "Food Delivery App Project.txt",
            icon: "/public/images/txt.png",
            kind: "file",
            fileType: "txt",
            position: "top-5 left-10",
            description: [
              "Our Food Delivery App is a fast and convenient way to order meals from your favorite restaurants.",
              "Instead of making calls or waiting in line, you can browse menus, customize orders, and track deliveries in real time.",
              "Think of it like having your favorite restaurants in your pocket—ready to deliver anytime, anywhere.",
              "It's built with React Native, so it works smoothly on both iOS and Android with a clean, modern design.",
            ],
          },
          {
            id: 2,
            name: "food-delivery-app.com",
            icon: "/public/images/safari.png",
            kind: "file",
            fileType: "url",
            href: "https://youtu.be/LKrX390fJMw?si=cExkuVhf2DTV9G2-",
            position: "top-10 right-20",
          },
          {
            id: 4,
            name: "food-delivery-app.png",
            icon: "/public/images/image.png",
            kind: "file",
            fileType: "img",
            position: "top-52 right-80",
            imageUrl: "/public/images/project-3.png",
          },
          {
            id: 5,
            name: "Design.fig",
            icon: "/public/images/plain.png",
            kind: "file",
            fileType: "fig",
            href: "https://google.com",
            position: "top-60 right-20",
          },
        ],
      },
      {
        id: 99,
        name: ".secret",
        icon: "/public/images/folder.png",
        kind: "folder",
        position: "top-10 right-5",
        children: [
          {
            id: 1,
            name: "secrets.txt",
            icon: "/public/images/txt.png",
            kind: "file",
            fileType: "txt",
            position: "top-5 left-10",
            description: [
              "🎉 Congratulations! You found the secret folder!",
              "Here are some hidden features you might have missed:",
              "• Try the Konami code: ↑↑↓↓←→←→BA",
              "• Click the Apple logo for 'About This Mac'",
              "• Double-click the Trash icon in the dock for a surprise",
              "Thanks for exploring! This portfolio was crafted with attention to every detail.",
            ],
          },
          {
            id: 2,
            name: "thank-you.txt",
            icon: "/public/images/txt.png",
            kind: "file",
            fileType: "txt",
            position: "top-20 right-20",
            description: [
              "Thank you for taking the time to explore this portfolio!",
              "Every interaction, animation, and detail was carefully crafted to create an immersive experience.",
              "If you made it this far, you clearly have an eye for detail too.",
              "Let's connect! Check out the contact section to get in touch.",
              "— Built with vanilla HTML, CSS, and JavaScript. No frameworks harmed in the making of this portfolio. 🌿",
            ],
          },
        ],
      },
    ],
  },
  about: {
    id: 2,
    type: "about",
    name: "About me",
    icon: "/public/icons/info.svg",
    kind: "folder",
    children: [
      {
        id: 1,
        name: "me.png",
        icon: "/public/images/image.png",
        kind: "file",
        fileType: "img",
        position: "top-10 left-5",
        imageUrl: "/public/images/adrian.jpg",
      },
      {
        id: 2,
        name: "casual-me.png",
        icon: "/public/images/image.png",
        kind: "file",
        fileType: "img",
        position: "top-28 right-72",
        imageUrl: "/public/images/adrian-2.jpg",
      },
      {
        id: 3,
        name: "conference-me.png",
        icon: "/public/images/image.png",
        kind: "file",
        fileType: "img",
        position: "top-52 left-80",
        imageUrl: "/public/images/adrian-3.jpeg",
      },
      {
        id: 4,
        name: "about-me.txt",
        icon: "/public/images/txt.png",
        kind: "file",
        fileType: "txt",
        position: "top-60 left-5",
        subtitle: "Meet the Developer Behind the Code",
        image: "/public/images/adrian.jpg",
        description: [
          "Hey! I'm Adrian 👋, a web developer who enjoys building sleek, interactive websites that actually work well.",
          "I specialize in JavaScript, React, and Next.js—and I love making things feel smooth, fast, and just a little bit delightful.",
          "I'm big on clean UI, good UX, and writing code that doesn't need a search party to debug.",
          "Outside of dev work, you'll find me tweaking layouts at 2AM, sipping overpriced coffee, or impulse-buying gadgets I absolutely convinced myself I needed 😅",
        ],
      },
      {
        id: 5,
        name: "fun-facts.txt",
        icon: "/public/images/txt.png",
        kind: "file",
        fileType: "txt",
        position: "top-10 right-10",
        description: [
          "📊 Fun Facts About This Portfolio:",
          "",
          "• Lines of CSS: 1000+",
          "• Lines of JavaScript: 800+",
          "• Easter eggs hidden: 5+",
          "• Frameworks used: 0 (pure vanilla!)",
          "• Coffee consumed during development: ∞",
          "• Attention to detail level: Obsessive",
          "",
          "Every pixel was placed with purpose. Every animation timed to perfection. Every feature crafted to surprise and delight.",
        ],
      },
    ],
  },
  resume: {
    id: 3,
    type: "resume",
    name: "Resume",
    icon: "/public/icons/file.svg",
    kind: "folder",
    children: [
      {
        id: 1,
        name: "Resume.pdf",
        icon: "/public/images/pdf.png",
        kind: "file",
        fileType: "pdf",
      },
    ],
  },
  trash: {
    id: 4,
    type: "trash",
    name: "Trash",
    icon: "/public/icons/trash.svg",
    kind: "folder",
    children: [
      {
        id: 1,
        name: "trash1.png",
        icon: "/public/images/image.png",
        kind: "file",
        fileType: "img",
        position: "top-10 left-10",
        imageUrl: "/public/images/trash-1.png",
      },
      {
        id: 2,
        name: "trash2.png",
        icon: "/public/images/image.png",
        kind: "file",
        fileType: "img",
        position: "top-40 left-80",
        imageUrl: "/public/images/trash-2.png",
      },
      {
        id: 3,
        name: "old-portfolio-ideas.txt",
        icon: "/public/images/txt.png",
        kind: "file",
        fileType: "txt",
        position: "top-10 right-20",
        description: [
          "🗑️ Discarded Portfolio Ideas:",
          "",
          "❌ A plain HTML page with just a list of projects (too boring)",
          "❌ A terminal-only interface (too intimidating)",
          "❌ A 3D spaceship flying through projects (too much)",
          "❌ An 8-bit game where you collect skills (fun but impractical)",
          "❌ A Windows 95 replica (already been done)",
          "",
          "✅ A macOS-style desktop with incredible attention to detail (PERFECT!)",
          "",
          "Sometimes the best ideas come from knowing what NOT to do.",
        ],
      },
    ],
  },
};

const INITIAL_Z_INDEX = 1000;

const WINDOW_CONFIG = {
  finder: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
  contact: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
  resume: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
  safari: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
  photos: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
  terminal: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
  txtfile: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
  imgfile: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
  about: { isOpen: false, zIndex: INITIAL_Z_INDEX, data: null },
};

// ============================================
// Application Code
// ============================================

// State Management
let windowState = JSON.parse(JSON.stringify(WINDOW_CONFIG));
let currentZIndex = INITIAL_Z_INDEX;
let navigationHistory = {};
let currentPaths = {};
let spotlightOpen = false;
let selectedSpotlightIndex = 0;
let spotlightResults = [];

// Konami Code state
const konamiCode = [
  "ArrowUp",
  "ArrowUp",
  "ArrowDown",
  "ArrowDown",
  "ArrowLeft",
  "ArrowRight",
  "ArrowLeft",
  "ArrowRight",
  "KeyB",
  "KeyA",
];
let konamiIndex = 0;

// DOM Elements
const dockContainer = document.getElementById("dock-container");
const menuTime = document.getElementById("menu-time");
const activeAppName = document.querySelector(".active-app");
const desktop = document.getElementById("desktop");
const spotlightOverlay = document.getElementById("spotlight-overlay");
const spotlightInput = document.getElementById("spotlight-input");
const spotlightResultsEl = document.getElementById("spotlight-results");
const contextMenu = document.getElementById("context-menu");

// Notification system
function showNotification(
  title,
  body,
  icon = "/public/images/finder.png",
  duration = 5000,
) {
  const notificationCenter = document.getElementById("notification-center");
  const notification = document.createElement("div");
  notification.className = "notification";
  notification.innerHTML = `
    <div class="notification-header">
      <img src="${icon}" alt="" class="notification-icon" />
      <span class="notification-app">Portfolio</span>
      <span class="notification-time">now</span>
    </div>
    <div class="notification-title">${title}</div>
    <div class="notification-body">${body}</div>
  `;

  notificationCenter.appendChild(notification);

  // Trigger animation after append
  setTimeout(() => notification.classList.add("show"), 10);

  // Click to dismiss
  notification.addEventListener("click", () =>
    dismissNotification(notification),
  );

  // Auto dismiss
  setTimeout(() => dismissNotification(notification), duration);
}

function dismissNotification(notification) {
  if (!notification.parentNode) return;
  notification.classList.remove("show");
  notification.classList.add("hide");
  setTimeout(() => notification.remove(), 400);
}

function showWelcomeNotification() {
  setTimeout(() => {
    showNotification(
      "Welcome to my Portfolio!",
      "Click the search icon or Apple logo to explore!",
      "/public/images/finder.png",
      6000,
    );
  }, 800);
}

// Initialize the application
document.addEventListener("DOMContentLoaded", () => {
  desktop.classList.add("loaded");

  initializeDock();
  initializeMenuBar();
  initializeWindows();
  initializeFinderSidebar();
  initializeSafari();
  initializePhotos();
  initializeContact();
  initializeTerminal();
  initializePdfViewer();
  initializeSpotlight();
  initializeContextMenu();
  initializeKeyboardShortcuts();
  initializeStatusTooltips();
  initializeAppleMenu();
  initializeMenuDropdowns();
  initializeKonamiCode();
  initializeDockMagnification();

  // Open Finder by default
  openWindow("finder");
  navigateToLocation("work");
});

// Menu Bar Time
function initializeMenuBar() {
  updateTime();
  setInterval(updateTime, 1000);
}

function updateTime() {
  const now = new Date();
  const options = {
    weekday: "short",
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  };
  if (menuTime) {
    menuTime.textContent = now.toLocaleDateString("en-US", options);
  }
}

// Apple Menu (About This Mac)
function initializeAppleMenu() {
  const appleLogo = document.getElementById("apple-menu");
  if (appleLogo) {
    appleLogo.addEventListener("click", (e) => {
      e.stopPropagation();
      openWindow("about");
    });
  }
}

// Menu Dropdowns
function initializeMenuDropdowns() {
  document.querySelectorAll(".dropdown-item").forEach((item) => {
    item.addEventListener("click", (e) => {
      e.stopPropagation();
      const action = item.dataset.action;
      if (action) handleMenuAction(action);
    });
  });
}

function handleMenuAction(action) {
  switch (action) {
    case "new-window":
      openWindow("finder");
      break;
    case "close-window":
      const focusedWindow = getFocusedWindow();
      if (focusedWindow) closeWindow(focusedWindow);
      break;
    case "go-work":
      openWindow("finder");
      navigateToLocation("work");
      break;
    case "go-about":
      openWindow("finder");
      navigateToLocation("about");
      break;
    case "go-resume":
      openWindow("finder");
      navigateToLocation("resume");
      break;
    case "go-trash":
      openWindow("finder");
      navigateToLocation("trash");
      break;
    case "minimize":
      const focused = getFocusedWindow();
      if (focused) minimizeWindow(focused);
      break;
    case "keyboard-shortcuts":
      showNotification(
        "Keyboard Shortcuts",
        "⌘+Space: Spotlight\n⌘+W: Close Window\n⌘+M: Minimize",
      );
      break;
    case "about-portfolio":
      openWindow("about");
      break;
    case "easter-eggs":
      showNotification(
        "Easter Eggs Hint",
        "Try the Konami code... or double-click the trash!",
      );
      break;
  }
}

function getFocusedWindow() {
  for (const [id, state] of Object.entries(windowState)) {
    if (state.isOpen) {
      const win = document.getElementById(`${id}-window`);
      if (win && win.classList.contains("focused")) {
        return id;
      }
    }
  }
  return null;
}

// Status Tooltips (WiFi, Battery)
function initializeStatusTooltips() {
  const wifiWrapper = document.getElementById("wifi-wrapper");
  const batteryWrapper = document.getElementById("battery-wrapper");
  const wifiTooltip = document.getElementById("wifi-tooltip");
  const batteryTooltip = document.getElementById("battery-tooltip");

  function closeAllTooltips() {
    wifiTooltip?.classList.remove("active");
    batteryTooltip?.classList.remove("active");
  }

  if (wifiWrapper && wifiTooltip) {
    wifiWrapper.addEventListener("click", (e) => {
      e.stopPropagation();
      const isActive = wifiTooltip.classList.contains("active");
      closeAllTooltips();
      if (!isActive) wifiTooltip.classList.add("active");
    });
  }

  if (batteryWrapper && batteryTooltip) {
    batteryWrapper.addEventListener("click", (e) => {
      e.stopPropagation();
      const isActive = batteryTooltip.classList.contains("active");
      closeAllTooltips();
      if (!isActive) batteryTooltip.classList.add("active");
    });
  }

  document.addEventListener("click", closeAllTooltips);

  // Get real battery status
  initializeBatteryStatus();
}

// Battery Status API
function initializeBatteryStatus() {
  if (!navigator.getBattery) {
    // Show message for unsupported browsers (Safari, Firefox)
    const timeRemainingEl = document
      .querySelectorAll("#battery-tooltip .status-row")[2]
      ?.querySelector(".status-value");
    if (timeRemainingEl) {
      timeRemainingEl.textContent = "Use Chrome for live data";
    }
    return;
  }

  navigator.getBattery().then((battery) => {
    updateBatteryUI(battery);

    // Listen for changes
    battery.addEventListener("levelchange", () => updateBatteryUI(battery));
    battery.addEventListener("chargingchange", () => updateBatteryUI(battery));
  });
}

function updateBatteryUI(battery) {
  const level = Math.round(battery.level * 100);
  const isCharging = battery.charging;

  // Update tooltip values
  const batteryLevelEl = document.querySelector(
    "#battery-tooltip .battery-level",
  );
  const batteryPercentEl = document.querySelector(
    "#battery-tooltip .status-battery .status-value",
  );
  const powerSourceEl = document
    .querySelectorAll("#battery-tooltip .status-row")[1]
    ?.querySelector(".status-value");
  const timeRemainingEl = document
    .querySelectorAll("#battery-tooltip .status-row")[2]
    ?.querySelector(".status-value");

  if (batteryLevelEl) {
    batteryLevelEl.style.width = `${level}%`;
    // Change color based on level
    if (level <= 20) {
      batteryLevelEl.style.background = "var(--control-close)";
    } else if (level <= 40) {
      batteryLevelEl.style.background = "var(--accent-orange)";
    } else {
      batteryLevelEl.style.background = "var(--accent-green)";
    }
  }

  if (batteryPercentEl) {
    batteryPercentEl.textContent = `${level}%`;
  }

  if (powerSourceEl) {
    powerSourceEl.textContent = isCharging ? "Power Adapter" : "Battery";
  }

  if (timeRemainingEl) {
    if (isCharging) {
      if (battery.chargingTime === Infinity) {
        timeRemainingEl.textContent = "Calculating...";
      } else {
        const hours = Math.floor(battery.chargingTime / 3600);
        const mins = Math.floor((battery.chargingTime % 3600) / 60);
        timeRemainingEl.textContent =
          hours > 0
            ? `${hours}:${String(mins).padStart(2, "0")} until full`
            : `${mins}m until full`;
      }
    } else {
      if (battery.dischargingTime === Infinity) {
        timeRemainingEl.textContent = "Calculating...";
      } else {
        const hours = Math.floor(battery.dischargingTime / 3600);
        const mins = Math.floor((battery.dischargingTime % 3600) / 60);
        timeRemainingEl.textContent =
          hours > 0 ? `${hours}:${String(mins).padStart(2, "0")}` : `${mins}m`;
      }
    }
  }
}

// Spotlight Search
function initializeSpotlight() {
  const trigger = document.getElementById("spotlight-trigger");

  if (trigger) {
    trigger.addEventListener("click", (e) => {
      e.stopPropagation();
      openSpotlight();
    });
  }

  spotlightOverlay?.addEventListener("click", (e) => {
    if (e.target === spotlightOverlay) {
      closeSpotlight();
    }
  });

  spotlightInput?.addEventListener("input", (e) => {
    handleSpotlightSearch(e.target.value);
  });

  spotlightInput?.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      closeSpotlight();
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedSpotlightIndex = Math.min(
        selectedSpotlightIndex + 1,
        spotlightResults.length - 1,
      );
      updateSpotlightSelection();
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedSpotlightIndex = Math.max(selectedSpotlightIndex - 1, 0);
      updateSpotlightSelection();
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (spotlightResults[selectedSpotlightIndex]) {
        executeSpotlightAction(spotlightResults[selectedSpotlightIndex]);
      }
    }
  });
}

function openSpotlight() {
  spotlightOpen = true;
  spotlightOverlay?.classList.add("active");
  if (spotlightInput) {
    spotlightInput.value = "";
    setTimeout(() => spotlightInput.focus(), 50);
  }
  handleSpotlightSearch("");
}

function closeSpotlight() {
  spotlightOpen = false;
  spotlightOverlay?.classList.remove("active");
  if (spotlightInput) spotlightInput.value = "";
  if (spotlightResultsEl) spotlightResultsEl.innerHTML = "";
}

function handleSpotlightSearch(query) {
  const results = [];
  const lowerQuery = query.toLowerCase().trim();

  // Search apps
  dockApps.forEach((app) => {
    if (lowerQuery === "" || app.name.toLowerCase().includes(lowerQuery)) {
      results.push({
        type: "app",
        title: app.name,
        subtitle: "Application",
        icon: `/public/images/${app.icon}`,
        action: () => {
          if (app.canOpen) {
            openWindow(app.id);
          } else if (app.id === "trash") {
            openWindow("finder");
            navigateToLocation("trash");
          }
        },
      });
    }
  });

  // Search locations
  Object.entries(locations).forEach(([key, location]) => {
    if (lowerQuery === "" || location.name.toLowerCase().includes(lowerQuery)) {
      results.push({
        type: "folder",
        title: location.name,
        subtitle: "Folder",
        icon: location.icon,
        action: () => {
          openWindow("finder");
          navigateToLocation(key);
        },
      });
    }

    // Search files within locations
    if (lowerQuery !== "") {
      location.children?.forEach((child) => {
        if (child.name.toLowerCase().includes(lowerQuery)) {
          results.push({
            type: "file",
            title: child.name,
            subtitle: `In ${location.name}`,
            icon: child.icon,
            action: () => {
              openWindow("finder");
              navigateToLocation(key);
              if (child.kind === "folder") {
                setTimeout(() => navigateToFolder(child, key), 100);
              }
            },
          });
        }
      });
    }
  });

  // Special commands
  if (
    lowerQuery !== "" &&
    ("about".includes(lowerQuery) || "mac".includes(lowerQuery))
  ) {
    results.push({
      type: "system",
      title: "About This Mac",
      subtitle: "System Information",
      icon: "/public/images/logo.svg",
      action: () => openWindow("about"),
    });
  }

  spotlightResults = results.slice(0, 8);
  selectedSpotlightIndex = 0;
  renderSpotlightResults();
}

function renderSpotlightResults() {
  if (!spotlightResultsEl) return;

  if (spotlightResults.length === 0) {
    spotlightResultsEl.innerHTML = `
      <div class="spotlight-section">
        <div class="spotlight-section-title">No Results</div>
      </div>
    `;
    return;
  }

  // Group by type
  const grouped = {};
  spotlightResults.forEach((result, index) => {
    result.index = index;
    if (!grouped[result.type]) grouped[result.type] = [];
    grouped[result.type].push(result);
  });

  const typeLabels = {
    app: "Applications",
    folder: "Folders",
    file: "Files",
    system: "System",
  };

  let html = "";
  Object.entries(grouped).forEach(([type, items]) => {
    html += `<div class="spotlight-section">
      <div class="spotlight-section-title">${typeLabels[type] || type}</div>
      ${items
        .map(
          (item) => `
        <div class="spotlight-item ${item.index === selectedSpotlightIndex ? "selected" : ""}" data-index="${item.index}">
          <img src="${item.icon}" alt="" class="spotlight-item-icon" />
          <div class="spotlight-item-text">
            <div class="spotlight-item-title">${item.title}</div>
            <div class="spotlight-item-subtitle">${item.subtitle}</div>
          </div>
        </div>
      `,
        )
        .join("")}
    </div>`;
  });

  spotlightResultsEl.innerHTML = html;

  // Add click handlers
  spotlightResultsEl.querySelectorAll(".spotlight-item").forEach((item) => {
    item.addEventListener("click", () => {
      const index = parseInt(item.dataset.index);
      executeSpotlightAction(spotlightResults[index]);
    });
  });
}

function updateSpotlightSelection() {
  spotlightResultsEl?.querySelectorAll(".spotlight-item").forEach((item) => {
    const index = parseInt(item.dataset.index);
    item.classList.toggle("selected", index === selectedSpotlightIndex);
  });
}

function executeSpotlightAction(result) {
  closeSpotlight();
  result.action();
}

// Context Menu
function initializeContextMenu() {
  document.addEventListener("contextmenu", (e) => {
    e.preventDefault();
    showContextMenu(e.clientX, e.clientY, e.target);
  });

  document.addEventListener("click", () => {
    contextMenu?.classList.remove("active");
  });
}

function showContextMenu(x, y, target) {
  if (!contextMenu) return;

  const items = getContextMenuItems(target);

  contextMenu.innerHTML = items
    .map((item) => {
      if (item.divider) {
        return '<div class="context-menu-divider"></div>';
      }
      return `
      <div class="context-menu-item ${item.disabled ? "disabled" : ""}" data-action="${item.action || ""}">
        ${item.label}
      </div>
    `;
    })
    .join("");

  // Position the menu
  const menuWidth = 180;
  const menuHeight = contextMenu.offsetHeight || 150;

  let posX = x;
  let posY = y;

  if (x + menuWidth > window.innerWidth) {
    posX = window.innerWidth - menuWidth - 10;
  }
  if (y + menuHeight > window.innerHeight) {
    posY = window.innerHeight - menuHeight - 10;
  }

  contextMenu.style.left = `${posX}px`;
  contextMenu.style.top = `${posY}px`;
  contextMenu.classList.add("active");

  // Add click handlers
  contextMenu
    .querySelectorAll(".context-menu-item:not(.disabled)")
    .forEach((item) => {
      item.addEventListener("click", (e) => {
        e.stopPropagation();
        handleContextAction(item.dataset.action, target);
        contextMenu.classList.remove("active");
      });
    });
}

function getContextMenuItems(target) {
  const isFile = target.closest(".file-item");
  const isDesktop =
    target.closest("#desktop") &&
    !target.closest(".window") &&
    !target.closest("#dock") &&
    !target.closest("#menu-bar");

  if (isFile) {
    return [
      { label: "Open", action: "open" },
      { label: "Get Info", action: "get-info" },
      { divider: true },
      { label: "Move to Trash", action: "trash", disabled: true },
    ];
  }

  if (isDesktop) {
    return [
      { label: "New Folder", action: "new-folder", disabled: true },
      { divider: true },
      { label: "Get Info", action: "desktop-info" },
      { label: "Change Wallpaper", action: "wallpaper", disabled: true },
    ];
  }

  return [{ label: "Refresh", action: "refresh" }];
}

function handleContextAction(action, target) {
  switch (action) {
    case "open":
      const fileItem = target.closest(".file-item");
      if (fileItem) {
        fileItem.dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
      }
      break;
    case "get-info":
      showNotification(
        "File Info",
        "This would show detailed file information.",
      );
      break;
    case "desktop-info":
      showNotification("Desktop", "macOS Portfolio Desktop v1.0.0");
      break;
    case "refresh":
      location.reload();
      break;
  }
}

// Keyboard Shortcuts (minimal - only Escape to close things)
function initializeKeyboardShortcuts() {
  document.addEventListener("keydown", (e) => {
    if (e.code === "Escape") {
      if (spotlightOpen) {
        closeSpotlight();
      }
      contextMenu?.classList.remove("active");
    }
  });
}

// Konami Code Easter Egg
function initializeKonamiCode() {
  document.addEventListener("keydown", (e) => {
    if (spotlightOpen) return;

    if (e.code === konamiCode[konamiIndex]) {
      konamiIndex++;
      if (konamiIndex === konamiCode.length) {
        activateKonamiCode();
        konamiIndex = 0;
      }
    } else {
      konamiIndex = e.code === konamiCode[0] ? 1 : 0;
    }
  });
}

function activateKonamiCode() {
  const isActive = document.body.classList.toggle("konami-active");

  if (isActive) {
    showNotification(
      "Achievement Unlocked!",
      "You found the Konami Code! Rainbow mode activated.",
      "/public/images/finder.png",
      5000,
    );
    createConfetti();
  } else {
    showNotification(
      "Rainbow Mode",
      "Deactivated. Enter the code again to re-enable!",
    );
  }
}

function createConfetti() {
  const colors = [
    "#ff6b6b",
    "#ffd93d",
    "#6bcb77",
    "#4d96ff",
    "#9b59b6",
    "#ff6b9d",
  ];

  // Add confetti animation style if not exists
  if (!document.getElementById("confetti-styles")) {
    const style = document.createElement("style");
    style.id = "confetti-styles";
    style.textContent = `
      @keyframes confettiFall {
        0% { transform: translateY(0) rotate(0deg); opacity: 1; }
        100% { transform: translateY(100vh) rotate(720deg); opacity: 0; }
      }
    `;
    document.head.appendChild(style);
  }

  for (let i = 0; i < 80; i++) {
    const confetti = document.createElement("div");
    confetti.style.cssText = `
      position: fixed;
      width: ${Math.random() * 10 + 6}px;
      height: ${Math.random() * 10 + 6}px;
      background: ${colors[Math.floor(Math.random() * colors.length)]};
      left: ${Math.random() * 100}vw;
      top: -20px;
      border-radius: ${Math.random() > 0.5 ? "50%" : "2px"};
      pointer-events: none;
      z-index: 999999;
      animation: confettiFall ${Math.random() * 2 + 2}s linear forwards;
    `;
    document.body.appendChild(confetti);
    setTimeout(() => confetti.remove(), 4000);
  }
}

// Dock Magnification Effect - Smooth Animation
function initializeDockMagnification() {
  const dock = document.getElementById("dock-container");
  if (!dock) return;

  const dockItems = dock.querySelectorAll(".dock-item");
  const itemStates = new Map();

  // Initialize state for each dock item
  dockItems.forEach((item) => {
    itemStates.set(item, {
      currentScale: 1,
      currentTranslateY: 0,
      targetScale: 1,
      targetTranslateY: 0,
    });
  });

  let mouseX = 0;
  let isHovering = false;
  let animationId = null;

  // Easing factor - lower = smoother but slower
  const easingFactor = 0.15;
  const maxDistance = 100;
  const maxScale = 1.45;
  const maxTranslateY = -18;

  function updateTargets() {
    dockItems.forEach((item) => {
      if (item.classList.contains("dock-divider")) return;

      const state = itemStates.get(item);
      if (!state) return;

      if (!isHovering) {
        state.targetScale = 1;
        state.targetTranslateY = 0;
        return;
      }

      const itemRect = item.getBoundingClientRect();
      const itemCenterX = itemRect.left + itemRect.width / 2;
      const distance = Math.abs(mouseX - itemCenterX);

      if (distance < maxDistance) {
        // Use a smoother curve (ease-out quad)
        const ratio = 1 - distance / maxDistance;
        const easedRatio = ratio * (2 - ratio); // Quadratic ease-out

        state.targetScale = 1 + (maxScale - 1) * easedRatio;
        state.targetTranslateY = maxTranslateY * easedRatio;
      } else {
        state.targetScale = 1;
        state.targetTranslateY = 0;
      }
    });
  }

  function animate() {
    let needsUpdate = false;

    dockItems.forEach((item) => {
      if (item.classList.contains("dock-divider")) return;

      const state = itemStates.get(item);
      if (!state) return;

      // Lerp towards target
      const scaleDiff = state.targetScale - state.currentScale;
      const translateDiff = state.targetTranslateY - state.currentTranslateY;

      // Check if we need to keep animating
      if (Math.abs(scaleDiff) > 0.001 || Math.abs(translateDiff) > 0.01) {
        needsUpdate = true;
        state.currentScale += scaleDiff * easingFactor;
        state.currentTranslateY += translateDiff * easingFactor;
      } else {
        state.currentScale = state.targetScale;
        state.currentTranslateY = state.targetTranslateY;
      }

      // Apply transform
      if (state.currentScale !== 1 || state.currentTranslateY !== 0) {
        item.style.transform = `translateY(${state.currentTranslateY}px) scale(${state.currentScale})`;
      } else {
        item.style.transform = "";
      }
    });

    if (needsUpdate || isHovering) {
      animationId = requestAnimationFrame(animate);
    } else {
      animationId = null;
    }
  }

  function startAnimation() {
    if (!animationId) {
      animationId = requestAnimationFrame(animate);
    }
  }

  dock.addEventListener("mouseenter", () => {
    isHovering = true;
    startAnimation();
  });

  dock.addEventListener("mousemove", (e) => {
    mouseX = e.clientX;
    updateTargets();
    startAnimation();
  });

  dock.addEventListener("mouseleave", () => {
    isHovering = false;
    updateTargets();
    startAnimation();
  });
}

// Dock Functionality
function initializeDock() {
  if (!dockContainer) return;

  dockApps.forEach((app, index) => {
    // Add divider before trash
    if (index === dockApps.length - 1) {
      const divider = document.createElement("div");
      divider.className = "dock-divider";
      dockContainer.appendChild(divider);
    }

    const dockItem = document.createElement("div");
    dockItem.className = "dock-item";
    dockItem.dataset.app = app.id;

    dockItem.innerHTML = `
      <span class="dock-tooltip">${app.name}</span>
      <img src="/public/images/${app.icon}" alt="${app.name}" class="dock-icon">
      <div class="dock-indicator"></div>
    `;

    if (app.canOpen) {
      dockItem.addEventListener("click", () => handleDockClick(app.id));
    } else if (app.id === "trash") {
      dockItem.addEventListener("click", () => {
        openWindow("finder");
        navigateToLocation("trash");
      });

      // Easter egg: double-click trash
      let clickCount = 0;
      dockItem.addEventListener("click", () => {
        clickCount++;
        if (clickCount === 2) {
          showNotification(
            "Empty Trash",
            "Recycling old projects... Just kidding, they're archived!",
            "/public/images/trash.png",
          );
          dockItem.classList.add("shake");
          setTimeout(() => dockItem.classList.remove("shake"), 400);
        }
        setTimeout(() => (clickCount = 0), 400);
      });
    }

    dockContainer.appendChild(dockItem);
  });
}

function handleDockClick(appId) {
  if (windowState[appId]?.isOpen) {
    focusWindow(appId);
  } else {
    openWindow(appId);
    if (appId === "finder" && !currentPaths["finder"]) {
      navigateToLocation("work");
    }
  }
}

function updateDockIndicator(appId, isOpen) {
  const dockItem = document.querySelector(`.dock-item[data-app="${appId}"]`);
  if (dockItem) {
    dockItem.classList.toggle("open", isOpen);
  }
}

// Window Management
function initializeWindows() {
  document.querySelectorAll(".window").forEach((windowEl) => {
    const windowId = windowEl.dataset.window;

    // Window controls
    windowEl.querySelectorAll(".control").forEach((control) => {
      control.addEventListener("click", (e) => {
        e.stopPropagation();
        const action = control.dataset.action;

        if (action === "close") closeWindow(windowId);
        else if (action === "minimize") minimizeWindow(windowId);
        else if (action === "maximize") maximizeWindow(windowId);
      });
    });

    // Click to focus
    windowEl.addEventListener("mousedown", () => focusWindow(windowId));

    // Dragging
    const header = windowEl.querySelector(".window-header");
    if (header) makeDraggable(windowEl, header);
  });
}

function openWindow(windowId) {
  const windowEl = document.getElementById(`${windowId}-window`);
  if (!windowEl) return;

  if (!windowState[windowId]) {
    windowState[windowId] = {
      isOpen: false,
      zIndex: INITIAL_Z_INDEX,
      data: null,
    };
  }

  windowState[windowId].isOpen = true;
  windowEl.classList.remove("closing", "minimizing");
  windowEl.classList.add("active");
  focusWindow(windowId);
  updateDockIndicator(windowId, true);
  updateActiveApp(windowId);
}

function closeWindow(windowId) {
  const windowEl = document.getElementById(`${windowId}-window`);
  if (!windowEl || !windowState[windowId]?.isOpen) return;

  windowEl.classList.add("closing");

  setTimeout(() => {
    windowState[windowId].isOpen = false;
    windowEl.classList.remove("active", "focused", "closing");
    updateDockIndicator(windowId, false);
    focusNextWindow();
  }, 200);
}

function minimizeWindow(windowId) {
  const windowEl = document.getElementById(`${windowId}-window`);
  if (!windowEl || !windowState[windowId]?.isOpen) return;

  windowEl.classList.add("minimizing");

  setTimeout(() => {
    windowState[windowId].isOpen = false;
    windowEl.classList.remove("active", "focused", "minimizing");
    updateDockIndicator(windowId, false);
    focusNextWindow();
  }, 350);
}

function focusNextWindow() {
  const openWindows = Object.entries(windowState)
    .filter(([id, state]) => state.isOpen)
    .sort((a, b) => b[1].zIndex - a[1].zIndex);

  if (openWindows.length > 0) {
    focusWindow(openWindows[0][0]);
  } else {
    updateActiveApp("finder");
  }
}

function maximizeWindow(windowId) {
  const windowEl = document.getElementById(`${windowId}-window`);
  if (!windowEl) return;

  const isMaximized = windowEl.dataset.maximized === "true";

  if (isMaximized) {
    windowEl.style.transition = "all 0.25s cubic-bezier(0.16, 1, 0.3, 1)";
    windowEl.style.width = windowEl.dataset.prevWidth;
    windowEl.style.height = windowEl.dataset.prevHeight;
    windowEl.style.top = windowEl.dataset.prevTop;
    windowEl.style.left = windowEl.dataset.prevLeft;
    windowEl.dataset.maximized = "false";
  } else {
    windowEl.dataset.prevWidth = windowEl.offsetWidth + "px";
    windowEl.dataset.prevHeight = windowEl.offsetHeight + "px";
    windowEl.dataset.prevTop = windowEl.offsetTop + "px";
    windowEl.dataset.prevLeft = windowEl.offsetLeft + "px";

    windowEl.style.transition = "all 0.25s cubic-bezier(0.16, 1, 0.3, 1)";
    windowEl.style.width = "calc(100vw - 20px)";
    windowEl.style.height = `calc(100vh - 108px)`;
    windowEl.style.top = "10px";
    windowEl.style.left = "10px";
    windowEl.dataset.maximized = "true";
  }

  setTimeout(() => {
    windowEl.style.transition = "";
  }, 250);
}

function focusWindow(windowId) {
  document
    .querySelectorAll(".window")
    .forEach((w) => w.classList.remove("focused"));

  const windowEl = document.getElementById(`${windowId}-window`);
  if (windowEl && windowState[windowId]) {
    currentZIndex++;
    windowEl.style.zIndex = currentZIndex;
    windowState[windowId].zIndex = currentZIndex;
    windowEl.classList.add("focused");
    updateActiveApp(windowId);
  }
}

function updateActiveApp(windowId) {
  const appNames = {
    finder: "Finder",
    safari: "Safari",
    photos: "Photos",
    contact: "Contacts",
    terminal: "Terminal",
    resume: "Preview",
    txtfile: "TextEdit",
    imgfile: "Preview",
    about: "Finder",
  };
  if (activeAppName) {
    activeAppName.textContent = appNames[windowId] || "Finder";
  }
}

function makeDraggable(element, handle) {
  let isDragging = false;
  let startX, startY, initialX, initialY;

  handle.addEventListener("mousedown", (e) => {
    if (e.target.closest(".control") || e.target.closest(".nav-btn")) return;

    isDragging = true;
    startX = e.clientX;
    startY = e.clientY;
    initialX = element.offsetLeft;
    initialY = element.offsetTop;

    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
  });

  function onMouseMove(e) {
    if (!isDragging) return;
    element.style.left = `${initialX + e.clientX - startX}px`;
    element.style.top = `${initialY + e.clientY - startY}px`;
  }

  function onMouseUp() {
    isDragging = false;
    document.removeEventListener("mousemove", onMouseMove);
    document.removeEventListener("mouseup", onMouseUp);
  }
}

// Finder Navigation
function initializeFinderSidebar() {
  const sidebarContainer = document.getElementById("finder-locations");
  if (!sidebarContainer) return;

  Object.entries(locations).forEach(([key, location]) => {
    const item = document.createElement("div");
    item.className = "sidebar-item";
    item.dataset.location = key;
    item.innerHTML = `
      <img src="${location.icon}" alt="${location.name}">
      <span>${location.name}</span>
    `;
    item.addEventListener("click", () => navigateToLocation(key));
    sidebarContainer.appendChild(item);
  });

  // Navigation buttons
  const backBtn = document.querySelector(
    '#finder-window .nav-btn[data-action="back"]',
  );
  const forwardBtn = document.querySelector(
    '#finder-window .nav-btn[data-action="forward"]',
  );

  backBtn?.addEventListener("click", () => navigateBack());
  forwardBtn?.addEventListener("click", () => navigateForward());

  navigationHistory["finder"] = { back: [], forward: [] };
}

function navigateToLocation(locationKey, pushHistory = true) {
  const location = locations[locationKey];
  if (!location) return;

  currentPaths["finder"] = { type: "location", key: locationKey };

  document
    .querySelectorAll("#finder-locations .sidebar-item")
    .forEach((item) => {
      item.classList.toggle("active", item.dataset.location === locationKey);
    });

  const titleText = document.querySelector("#finder-window .title-text");
  if (titleText) titleText.textContent = location.name;

  renderFinderContent(location.children, locationKey);

  if (pushHistory && navigationHistory["finder"]) {
    navigationHistory["finder"].back.push({
      type: "location",
      key: locationKey,
    });
    navigationHistory["finder"].forward = [];
  }

  updateNavigationButtons();
}

function navigateToFolder(folder, parentLocationKey, pushHistory = true) {
  currentPaths["finder"] = { type: "folder", folder, parentLocationKey };

  const titleText = document.querySelector("#finder-window .title-text");
  if (titleText) titleText.textContent = folder.name;

  document
    .querySelectorAll("#finder-locations .sidebar-item")
    .forEach((item) => {
      item.classList.remove("active");
    });

  renderFinderContent(folder.children, parentLocationKey, folder);

  if (pushHistory && navigationHistory["finder"]) {
    navigationHistory["finder"].back.push({
      type: "folder",
      folder,
      parentLocationKey,
    });
    navigationHistory["finder"].forward = [];
  }

  updateNavigationButtons();
}

function renderFinderContent(items, locationKey, parentFolder = null) {
  const content = document.getElementById("finder-content");
  if (!content) return;

  content.innerHTML = "";
  content.classList.remove("files-grid");

  // Add breadcrumb if in a folder
  if (parentFolder) {
    const breadcrumb = document.createElement("div");
    breadcrumb.className = "breadcrumb";
    const locationName = locations[locationKey]?.name || locationKey;
    breadcrumb.innerHTML = `
      <span class="breadcrumb-item" data-location="${locationKey}">${locationName}</span>
      <span class="breadcrumb-separator">›</span>
      <span class="breadcrumb-item current">${parentFolder.name}</span>
    `;
    breadcrumb
      .querySelector(`[data-location="${locationKey}"]`)
      ?.addEventListener("click", () => {
        navigateToLocation(locationKey);
      });
    content.appendChild(breadcrumb);
  }

  // Render items
  const hasPositions = items.some((item) => item.position);

  items.forEach((item) => {
    const fileItem = document.createElement("div");
    fileItem.className = "file-item";

    if (item.position && hasPositions) {
      item.position.split(" ").forEach((pos) => {
        const match = pos.match(/^(top|bottom|left|right)-(\d+)$/);
        if (match) {
          fileItem.style[match[1]] = `${parseInt(match[2]) * 4}px`;
        }
      });
    }

    fileItem.innerHTML = `
      <img src="${item.icon}" alt="${item.name}" class="file-item-icon">
      <span class="file-item-name">${item.name}</span>
    `;

    fileItem.addEventListener("dblclick", () =>
      handleFileOpen(item, locationKey),
    );
    fileItem.addEventListener("click", (e) => {
      e.stopPropagation();
      document
        .querySelectorAll(".file-item")
        .forEach((f) => f.classList.remove("selected"));
      fileItem.classList.add("selected");
    });

    content.appendChild(fileItem);
  });

  if (!hasPositions) {
    content.classList.add("files-grid");
  }
}

function handleFileOpen(item, locationKey) {
  if (item.kind === "folder") {
    navigateToFolder(item, locationKey);
    return;
  }

  switch (item.fileType) {
    case "txt":
      openTextFile(item);
      break;
    case "img":
      openImageFile(item);
      break;
    case "url":
      window.open(item.href, "_blank");
      break;
    case "pdf":
      openWindow("resume");
      break;
    case "fig":
      window.open(item.href, "_blank");
      break;
  }
}

function navigateBack() {
  const history = navigationHistory["finder"];
  if (!history || history.back.length <= 1) return;

  const current = history.back.pop();
  history.forward.push(current);
  const previous = history.back[history.back.length - 1];

  if (previous.type === "location") {
    navigateToLocation(previous.key, false);
  } else {
    navigateToFolder(previous.folder, previous.parentLocationKey, false);
  }
}

function navigateForward() {
  const history = navigationHistory["finder"];
  if (!history || history.forward.length === 0) return;

  const next = history.forward.pop();
  history.back.push(next);

  if (next.type === "location") {
    navigateToLocation(next.key, false);
  } else {
    navigateToFolder(next.folder, next.parentLocationKey, false);
  }
}

function updateNavigationButtons() {
  const history = navigationHistory["finder"];
  const backBtn = document.querySelector(
    '#finder-window .nav-btn[data-action="back"]',
  );
  const forwardBtn = document.querySelector(
    '#finder-window .nav-btn[data-action="forward"]',
  );

  if (backBtn) backBtn.disabled = !history || history.back.length <= 1;
  if (forwardBtn)
    forwardBtn.disabled = !history || history.forward.length === 0;
}

// Text File Viewer
function openTextFile(item) {
  openWindow("txtfile");

  const titleEl = document.getElementById("txt-title");
  const contentEl = document.getElementById("txt-content");

  if (titleEl) titleEl.textContent = item.name;

  let html = "";

  if (item.subtitle && item.image) {
    html += `
      <div class="txt-header">
        <img src="${item.image}" alt="" class="txt-header-image">
        <div class="txt-header-content">
          <div class="txt-subtitle">${item.subtitle}</div>
        </div>
      </div>
    `;
  }

  if (item.description && Array.isArray(item.description)) {
    item.description.forEach((para) => {
      html += `<p>${para}</p>`;
    });
  }

  if (contentEl) contentEl.innerHTML = html;
}

// Image File Viewer
function openImageFile(item) {
  openWindow("imgfile");

  const titleEl = document.getElementById("img-title");
  const imgEl = document.getElementById("img-preview");

  if (titleEl) titleEl.textContent = item.name;
  if (imgEl) {
    imgEl.src = item.imageUrl;
    imgEl.alt = item.name;
  }
}

// Safari Browser
function initializeSafari() {
  const urlInput = document.getElementById("safari-url-input");
  const iframe = document.getElementById("safari-iframe");
  const backBtn = document.getElementById("safari-back");
  const forwardBtn = document.getElementById("safari-forward");
  const reloadBtn = document.getElementById("safari-reload");

  if (!urlInput || !iframe) return;

  // Browser history for Safari
  let safariHistory = ["https://www.google.com/webhp?igu=1"];
  let safariHistoryIndex = 0;

  // Navigate to URL
  function navigateToUrl(url) {
    // Add protocol if missing
    if (!url.startsWith("http://") && !url.startsWith("https://")) {
      // Check if it looks like a URL or a search query
      if (url.includes(".") && !url.includes(" ")) {
        url = "https://" + url;
      } else {
        // Treat as Google search
        url =
          "https://www.google.com/search?igu=1&q=" + encodeURIComponent(url);
      }
    }

    // Update iframe
    iframe.src = url;
    urlInput.value = url;

    // Update history
    if (safariHistoryIndex < safariHistory.length - 1) {
      safariHistory = safariHistory.slice(0, safariHistoryIndex + 1);
    }
    safariHistory.push(url);
    safariHistoryIndex = safariHistory.length - 1;

    updateSafariNavButtons();
  }

  function updateSafariNavButtons() {
    if (backBtn) backBtn.disabled = safariHistoryIndex <= 0;
    if (forwardBtn)
      forwardBtn.disabled = safariHistoryIndex >= safariHistory.length - 1;
  }

  // URL input - navigate on Enter
  urlInput.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      e.preventDefault();
      navigateToUrl(urlInput.value.trim());
      urlInput.blur();
    }
  });

  // Select all on focus
  urlInput.addEventListener("focus", () => {
    setTimeout(() => urlInput.select(), 0);
  });

  // Back button
  backBtn?.addEventListener("click", () => {
    if (safariHistoryIndex > 0) {
      safariHistoryIndex--;
      const url = safariHistory[safariHistoryIndex];
      iframe.src = url;
      urlInput.value = url;
      updateSafariNavButtons();
    }
  });

  // Forward button
  forwardBtn?.addEventListener("click", () => {
    if (safariHistoryIndex < safariHistory.length - 1) {
      safariHistoryIndex++;
      const url = safariHistory[safariHistoryIndex];
      iframe.src = url;
      urlInput.value = url;
      updateSafariNavButtons();
    }
  });

  // Reload button
  reloadBtn?.addEventListener("click", () => {
    iframe.src = iframe.src;
  });

  // Make Safari browser available globally for opening URLs
  window.openInSafari = (url) => {
    openWindow("safari");
    setTimeout(() => navigateToUrl(url), 100);
  };
}

// Photos/Gallery
function initializePhotos() {
  const photosLinksContainer = document.getElementById("photos-links");
  if (photosLinksContainer) {
    photosLinks.forEach((link) => {
      const item = document.createElement("div");
      item.className = "sidebar-item";
      item.innerHTML = `
        <img src="${link.icon}" alt="${link.title}">
        <span>${link.title}</span>
      `;
      photosLinksContainer.appendChild(item);
    });
  }

  const galleryGrid = document.getElementById("gallery-grid");
  if (galleryGrid) {
    gallery.forEach((item) => {
      const galleryItem = document.createElement("div");
      galleryItem.className = "gallery-item";
      galleryItem.innerHTML = `<img src="${item.img}" alt="Gallery image">`;
      galleryItem.addEventListener("click", () => {
        openWindow("imgfile");
        const titleEl = document.getElementById("img-title");
        const imgEl = document.getElementById("img-preview");
        if (titleEl) titleEl.textContent = "Gallery Image";
        if (imgEl) imgEl.src = item.img;
      });
      galleryGrid.appendChild(galleryItem);
    });
  }
}

// Contact
function initializeContact() {
  const socialsGrid = document.getElementById("socials-grid");
  if (!socialsGrid) return;

  socials.forEach((social) => {
    const card = document.createElement("a");
    card.className = "social-card";
    card.href = social.link;
    card.target = "_blank";
    card.style.backgroundColor = social.bg;
    card.innerHTML = `
      <img src="${social.icon}" alt="${social.text}" class="social-icon">
      <span class="social-text">${social.text}</span>
    `;
    socialsGrid.appendChild(card);
  });
}

// Terminal/Skills
function initializeTerminal() {
  const terminalContent = document.getElementById("terminal-content");
  if (!terminalContent) return;

  function renderTerminal() {
    let html = `
      <div class="terminal-line">
        <span class="terminal-prompt">portfolio@mac ~ %</span>
        <span class="terminal-command"> cat skills.json</span>
      </div>
      <div class="terminal-output">
    `;

    techStack.forEach((category) => {
      html += `
        <div class="skill-category">
          <span class="skill-category-name">${category.category}:</span>
          <div class="skill-items">${category.items.join(", ")}</div>
        </div>
      `;
    });

    html += `
      </div>
      <div class="terminal-line">
        <span class="terminal-prompt">portfolio@mac ~ %</span>
        <span class="terminal-command terminal-cursor"> _</span>
      </div>
    `;

    terminalContent.innerHTML = html;
  }

  renderTerminal();
}

// PDF Viewer
let pdfDoc = null;
let pdfPageNum = 1;
let pdfScale = 1.5;
let pdfRendering = false;
let pdfPagePending = null;

function initializePdfViewer() {
  const canvas = document.getElementById("pdf-canvas");
  const ctx = canvas?.getContext("2d");
  const prevBtn = document.getElementById("pdf-prev");
  const nextBtn = document.getElementById("pdf-next");
  const pageInfo = document.getElementById("pdf-page-info");
  const zoomIn = document.getElementById("pdf-zoom-in");
  const zoomOut = document.getElementById("pdf-zoom-out");
  const zoomLevel = document.getElementById("pdf-zoom-level");

  if (!canvas || !ctx) return;

  // Set worker path
  pdfjsLib.GlobalWorkerOptions.workerSrc =
    "https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.worker.min.js";

  function renderPage(num) {
    pdfRendering = true;

    pdfDoc.getPage(num).then((page) => {
      const viewport = page.getViewport({ scale: pdfScale });
      canvas.height = viewport.height;
      canvas.width = viewport.width;

      // Resize window to fit PDF width and center it
      const resumeWindow = document.getElementById("resume-window");
      if (resumeWindow) {
        const newWidth = viewport.width + 40;
        resumeWindow.style.width = `${newWidth}px`;
        resumeWindow.style.left = `calc(50% - ${newWidth / 2}px)`;
        resumeWindow.style.transform = "none";
      }

      const renderContext = {
        canvasContext: ctx,
        viewport: viewport,
      };

      page.render(renderContext).promise.then(() => {
        pdfRendering = false;
        if (pdfPagePending !== null) {
          renderPage(pdfPagePending);
          pdfPagePending = null;
        }
      });
    });

    pageInfo.textContent = `Page ${num} of ${pdfDoc.numPages}`;
    prevBtn.disabled = num <= 1;
    nextBtn.disabled = num >= pdfDoc.numPages;
  }

  function queueRenderPage(num) {
    if (pdfRendering) {
      pdfPagePending = num;
    } else {
      renderPage(num);
    }
  }

  function updateZoomDisplay() {
    zoomLevel.textContent = `${Math.round((pdfScale * 100) / 1.5)}%`;
  }

  // Load PDF
  pdfjsLib.getDocument("/public/files/resume.pdf").promise.then((pdf) => {
    pdfDoc = pdf;
    pageInfo.textContent = `Page 1 of ${pdf.numPages}`;
    renderPage(pdfPageNum);
    updateZoomDisplay();
  });

  // Navigation
  prevBtn?.addEventListener("click", () => {
    if (pdfPageNum <= 1) return;
    pdfPageNum--;
    queueRenderPage(pdfPageNum);
  });

  nextBtn?.addEventListener("click", () => {
    if (pdfPageNum >= pdfDoc.numPages) return;
    pdfPageNum++;
    queueRenderPage(pdfPageNum);
  });

  // Zoom
  zoomIn?.addEventListener("click", () => {
    pdfScale += 0.25;
    updateZoomDisplay();
    queueRenderPage(pdfPageNum);
  });

  zoomOut?.addEventListener("click", () => {
    if (pdfScale <= 0.5) return;
    pdfScale -= 0.25;
    updateZoomDisplay();
    queueRenderPage(pdfPageNum);
  });
}

// ============================================
