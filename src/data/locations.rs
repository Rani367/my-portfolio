//! File system locations and structure data.

/// The type of file.
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum FileType {
    Txt,
    Img,
    Url,
    Pdf,
    Fig,
}

/// Whether an item is a folder or file.
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ItemKind {
    Folder,
    File,
}

/// Represents a file or folder in the virtual file system.
#[derive(Clone, Debug, PartialEq)]
pub struct FileItem {
    pub id: u32,
    pub name: &'static str,
    pub icon: &'static str,
    pub kind: ItemKind,
    pub file_type: Option<FileType>,
    pub position: Option<&'static str>,
    pub description: Option<&'static [&'static str]>,
    pub subtitle: Option<&'static str>,
    pub image: Option<&'static str>,
    pub image_url: Option<&'static str>,
    pub href: Option<&'static str>,
    pub children: Option<&'static [FileItem]>,
}

/// Represents a top-level location in the file system.
#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub id: u32,
    pub location_type: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub kind: ItemKind,
    pub children: &'static [FileItem],
}

// Helper macro to reduce boilerplate for creating FileItem
impl FileItem {
    pub const fn folder(
        id: u32,
        name: &'static str,
        icon: &'static str,
        position: Option<&'static str>,
        children: &'static [FileItem],
    ) -> Self {
        Self {
            id,
            name,
            icon,
            kind: ItemKind::Folder,
            file_type: None,
            position,
            description: None,
            subtitle: None,
            image: None,
            image_url: None,
            href: None,
            children: Some(children),
        }
    }

    pub const fn txt_file(
        id: u32,
        name: &'static str,
        position: Option<&'static str>,
        description: &'static [&'static str],
    ) -> Self {
        Self {
            id,
            name,
            icon: "/public/images/txt.png",
            kind: ItemKind::File,
            file_type: Some(FileType::Txt),
            position,
            description: Some(description),
            subtitle: None,
            image: None,
            image_url: None,
            href: None,
            children: None,
        }
    }

    pub const fn txt_file_with_image(
        id: u32,
        name: &'static str,
        position: Option<&'static str>,
        description: &'static [&'static str],
        subtitle: &'static str,
        image: &'static str,
    ) -> Self {
        Self {
            id,
            name,
            icon: "/public/images/txt.png",
            kind: ItemKind::File,
            file_type: Some(FileType::Txt),
            position,
            description: Some(description),
            subtitle: Some(subtitle),
            image: Some(image),
            image_url: None,
            href: None,
            children: None,
        }
    }

    pub const fn img_file(
        id: u32,
        name: &'static str,
        position: Option<&'static str>,
        image_url: &'static str,
    ) -> Self {
        Self {
            id,
            name,
            icon: "/public/images/image.png",
            kind: ItemKind::File,
            file_type: Some(FileType::Img),
            position,
            description: None,
            subtitle: None,
            image: None,
            image_url: Some(image_url),
            href: None,
            children: None,
        }
    }

    pub const fn url_file(
        id: u32,
        name: &'static str,
        position: Option<&'static str>,
        href: &'static str,
    ) -> Self {
        Self {
            id,
            name,
            icon: "/public/images/safari.png",
            kind: ItemKind::File,
            file_type: Some(FileType::Url),
            position,
            description: None,
            subtitle: None,
            image: None,
            image_url: None,
            href: Some(href),
            children: None,
        }
    }

    pub const fn fig_file(
        id: u32,
        name: &'static str,
        position: Option<&'static str>,
        href: &'static str,
    ) -> Self {
        Self {
            id,
            name,
            icon: "/public/images/plain.png",
            kind: ItemKind::File,
            file_type: Some(FileType::Fig),
            position,
            description: None,
            subtitle: None,
            image: None,
            image_url: None,
            href: Some(href),
            children: None,
        }
    }

    pub const fn pdf_file(id: u32, name: &'static str) -> Self {
        Self {
            id,
            name,
            icon: "/public/images/pdf.png",
            kind: ItemKind::File,
            file_type: Some(FileType::Pdf),
            position: None,
            description: None,
            subtitle: None,
            image: None,
            image_url: None,
            href: None,
            children: None,
        }
    }
}

// ============================================
// Project Children Data
// ============================================

const NIKE_PROJECT_CHILDREN: &[FileItem] = &[
    FileItem::txt_file(
        1,
        "Nike Project.txt",
        Some("top-5 left-10"),
        &[
            "The Nike eCommerce website is a sleek and modern platform designed for shopping the latest Nike collections.",
            "Instead of a simple online store, it delivers an immersive experience with bold visuals, interactive product displays, and smooth navigation.",
            "Think of it like walking into a flagship Nike store—but right from your phone or laptop.",
            "It's built with Next.js and Tailwind, ensuring fast performance, responsive design, and a clean, premium look.",
        ],
    ),
    FileItem::url_file(
        2,
        "nike.com",
        Some("top-10 right-20"),
        "https://youtu.be/fZdTYswuZjU?si=Awjl-pIst9e09_UU",
    ),
    FileItem::img_file(
        4,
        "nike.png",
        Some("top-52 right-80"),
        "/public/images/project-1.png",
    ),
    FileItem::fig_file(
        5,
        "Design.fig",
        Some("top-60 right-20"),
        "https://google.com",
    ),
];

const AI_RESUME_CHILDREN: &[FileItem] = &[
    FileItem::txt_file(
        1,
        "AI Resume Analyzer Project.txt",
        Some("top-5 right-10"),
        &[
            "AI Resume Analyzer is a smart tool that helps you perfect your resume with instant feedback.",
            "Instead of guessing what recruiters want, you get AI-powered insights on keywords, formatting, and overall impact.",
            "Think of it like having a career coach—pointing out strengths, fixing weaknesses, and boosting your chances of landing interviews.",
            "It's built with Next.js and Tailwind, so it runs fast, looks professional, and works seamlessly on any device.",
        ],
    ),
    FileItem::url_file(
        2,
        "ai-resume-analyzer.com",
        Some("top-20 left-20"),
        "https://youtu.be/iYOz165wGkQ?si=R1hs8Legl200m0Cl",
    ),
    FileItem::img_file(
        4,
        "ai-resume-analyzer.png",
        Some("top-52 left-80"),
        "/public/images/project-2.png",
    ),
    FileItem::fig_file(
        5,
        "Design.fig",
        Some("top-60 left-5"),
        "https://google.com",
    ),
];

const FOOD_DELIVERY_CHILDREN: &[FileItem] = &[
    FileItem::txt_file(
        1,
        "Food Delivery App Project.txt",
        Some("top-5 left-10"),
        &[
            "Our Food Delivery App is a fast and convenient way to order meals from your favorite restaurants.",
            "Instead of making calls or waiting in line, you can browse menus, customize orders, and track deliveries in real time.",
            "Think of it like having your favorite restaurants in your pocket—ready to deliver anytime, anywhere.",
            "It's built with React Native, so it works smoothly on both iOS and Android with a clean, modern design.",
        ],
    ),
    FileItem::url_file(
        2,
        "food-delivery-app.com",
        Some("top-10 right-20"),
        "https://youtu.be/LKrX390fJMw?si=cExkuVhf2DTV9G2-",
    ),
    FileItem::img_file(
        4,
        "food-delivery-app.png",
        Some("top-52 right-80"),
        "/public/images/project-3.png",
    ),
    FileItem::fig_file(
        5,
        "Design.fig",
        Some("top-60 right-20"),
        "https://google.com",
    ),
];

const SECRET_FOLDER_CHILDREN: &[FileItem] = &[
    FileItem::txt_file(
        1,
        "secrets.txt",
        Some("top-5 left-10"),
        &[
            "🎉 Congratulations! You found the secret folder!",
            "Here are some hidden features you might have missed:",
            "• Try the Konami code: ↑↑↓↓←→←→BA",
            "• Click the Apple logo for 'About This Mac'",
            "• Double-click the Trash icon in the dock for a surprise",
            "Thanks for exploring! This portfolio was crafted with attention to every detail.",
        ],
    ),
    FileItem::txt_file(
        2,
        "thank-you.txt",
        Some("top-20 right-20"),
        &[
            "Thank you for taking the time to explore this portfolio!",
            "Every interaction, animation, and detail was carefully crafted to create an immersive experience.",
            "If you made it this far, you clearly have an eye for detail too.",
            "Let's connect! Check out the contact section to get in touch.",
            "— Built with vanilla HTML, CSS, and JavaScript. No frameworks harmed in the making of this portfolio. 🌿",
        ],
    ),
];

// ============================================
// Work Location Children
// ============================================

const WORK_CHILDREN: &[FileItem] = &[
    FileItem::folder(
        5,
        "Nike Ecommerce Website Application",
        "/public/images/folder.png",
        Some("top-10 left-5"),
        NIKE_PROJECT_CHILDREN,
    ),
    FileItem::folder(
        6,
        "AI Resume Analyzer",
        "/public/images/folder.png",
        Some("top-52 right-80"),
        AI_RESUME_CHILDREN,
    ),
    FileItem::folder(
        7,
        "Food Delivery App",
        "/public/images/folder.png",
        Some("top-10 left-80"),
        FOOD_DELIVERY_CHILDREN,
    ),
    FileItem::folder(
        99,
        ".secret",
        "/public/images/folder.png",
        Some("top-10 right-5"),
        SECRET_FOLDER_CHILDREN,
    ),
];

// ============================================
// About Location Children
// ============================================

const ABOUT_CHILDREN: &[FileItem] = &[
    FileItem::img_file(
        1,
        "me.png",
        Some("top-10 left-5"),
        "/public/images/adrian.jpg",
    ),
    FileItem::img_file(
        2,
        "casual-me.png",
        Some("top-28 right-72"),
        "/public/images/adrian-2.jpg",
    ),
    FileItem::img_file(
        3,
        "conference-me.png",
        Some("top-52 left-80"),
        "/public/images/adrian-3.jpeg",
    ),
    FileItem::txt_file_with_image(
        4,
        "about-me.txt",
        Some("top-60 left-5"),
        &[
            "Hey! I'm Adrian 👋, a web developer who enjoys building sleek, interactive websites that actually work well.",
            "I specialize in JavaScript, React, and Next.js—and I love making things feel smooth, fast, and just a little bit delightful.",
            "I'm big on clean UI, good UX, and writing code that doesn't need a search party to debug.",
            "Outside of dev work, you'll find me tweaking layouts at 2AM, sipping overpriced coffee, or impulse-buying gadgets I absolutely convinced myself I needed 😅",
        ],
        "Meet the Developer Behind the Code",
        "/public/images/adrian.jpg",
    ),
    FileItem::txt_file(
        5,
        "fun-facts.txt",
        Some("top-10 right-10"),
        &[
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
    ),
];

// ============================================
// Resume Location Children
// ============================================

const RESUME_CHILDREN: &[FileItem] = &[
    FileItem::pdf_file(1, "Resume.pdf"),
];

// ============================================
// Trash Location Children
// ============================================

const TRASH_CHILDREN: &[FileItem] = &[
    FileItem::img_file(
        1,
        "trash1.png",
        Some("top-10 left-10"),
        "/public/images/trash-1.png",
    ),
    FileItem::img_file(
        2,
        "trash2.png",
        Some("top-40 left-80"),
        "/public/images/trash-2.png",
    ),
    FileItem::txt_file(
        3,
        "old-portfolio-ideas.txt",
        Some("top-10 right-20"),
        &[
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
    ),
];

// ============================================
// Top-Level Locations
// ============================================

/// Work location containing project folders.
pub const LOCATION_WORK: Location = Location {
    id: 1,
    location_type: "work",
    name: "Work",
    icon: "/public/icons/work.svg",
    kind: ItemKind::Folder,
    children: WORK_CHILDREN,
};

/// About location containing personal info.
pub const LOCATION_ABOUT: Location = Location {
    id: 2,
    location_type: "about",
    name: "About me",
    icon: "/public/icons/info.svg",
    kind: ItemKind::Folder,
    children: ABOUT_CHILDREN,
};

/// Resume location containing PDF resume.
pub const LOCATION_RESUME: Location = Location {
    id: 3,
    location_type: "resume",
    name: "Resume",
    icon: "/public/icons/file.svg",
    kind: ItemKind::Folder,
    children: RESUME_CHILDREN,
};

/// Trash location containing archived items.
pub const LOCATION_TRASH: Location = Location {
    id: 4,
    location_type: "trash",
    name: "Trash",
    icon: "/public/icons/trash.svg",
    kind: ItemKind::Folder,
    children: TRASH_CHILDREN,
};

/// All locations in the file system.
pub const LOCATIONS: &[Location] = &[
    LOCATION_WORK,
    LOCATION_ABOUT,
    LOCATION_RESUME,
    LOCATION_TRASH,
];

/// Get a location by its type name.
pub fn get_location(location_type: &str) -> Option<&'static Location> {
    LOCATIONS.iter().find(|loc| loc.location_type == location_type)
}
