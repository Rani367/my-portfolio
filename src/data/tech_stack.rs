//! Technology stack configuration data.

/// Represents a category of technologies.
#[derive(Clone, Debug, PartialEq)]
pub struct TechCategory {
    pub category: &'static str,
    pub items: &'static [&'static str],
}

/// All technology categories for the skills terminal.
pub const TECH_STACK: &[TechCategory] = &[
    TechCategory {
        category: "Frontend",
        items: &["React.js", "Next.js", "TypeScript"],
    },
    TechCategory {
        category: "Mobile",
        items: &["React Native", "Expo"],
    },
    TechCategory {
        category: "Styling",
        items: &["Tailwind CSS", "Sass", "CSS"],
    },
    TechCategory {
        category: "Backend",
        items: &["Node.js", "Express", "NestJS", "Hono"],
    },
    TechCategory {
        category: "Database",
        items: &["MongoDB", "PostgreSQL"],
    },
    TechCategory {
        category: "Dev Tools",
        items: &["Git", "GitHub", "Docker"],
    },
];
