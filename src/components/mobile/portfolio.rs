//! 3D Interactive Mobile Portfolio.
//!
//! A beautiful, scroll-reactive portfolio with 3D effects, parallax, and smooth animations.

use leptos::prelude::*;
use crate::data::{TECH_STACK, SOCIALS, LOCATION_WORK};
use crate::hooks::{use_scroll, use_element_in_view};

/// 3D Hero section with floating elements and parallax.
#[component]
fn HeroSection(scroll_y: ReadSignal<f64>) -> impl IntoView {
    // Parallax offset based on scroll
    let parallax_offset = Memo::new(move |_| scroll_y.get() * 0.3);
    let orb_offset = Memo::new(move |_| scroll_y.get() * 0.15);

    view! {
        <section class="mobile-hero-3d">
            // Floating background orbs with parallax
            <div
                class="hero-orb orb-1"
                style:transform=move || format!("translateY({}px)", orb_offset.get())
            />
            <div
                class="hero-orb orb-2"
                style:transform=move || format!("translateY({}px)", -orb_offset.get() * 0.5)
            />
            <div
                class="hero-orb orb-3"
                style:transform=move || format!("translateY({}px)", orb_offset.get() * 0.8)
            />

            <div class="hero-content-3d" style:transform=move || format!("translateY({}px)", parallax_offset.get())>
                <div class="hero-badge">"Software Engineer"</div>
                <h1 class="hero-name-3d">
                    <span class="name-line">"Rani"</span>
                    <span class="name-line">"Malach"</span>
                </h1>
                <p class="hero-tagline-3d">
                    "Building modern web experiences with"
                    <br />
                    <span class="highlight">"React"</span>", "
                    <span class="highlight">"TypeScript"</span>", and "
                    <span class="highlight">"Rust"</span>
                </p>
                <div class="hero-cta">
                    <a href="#projects" class="cta-btn primary">
                        <span>"View Work"</span>
                        <span class="btn-arrow">"↓"</span>
                    </a>
                </div>
            </div>

            // Scroll indicator
            <div class="scroll-indicator">
                <div class="scroll-line" />
            </div>
        </section>
    }
}

/// 3D Project card with depth and reveal animation.
#[component]
fn ProjectCard3D(
    name: &'static str,
    description: &'static str,
    image: &'static str,
    link: &'static str,
    index: usize,
) -> impl IntoView {
    let delay_ms = index * 100;

    view! {
        <div class="reveal-card project-reveal" style:transition-delay=format!("{}ms", delay_ms)>
            <a href=link target="_blank" rel="noopener" class="project-card-3d">
                <div class="card-inner">
                    <div class="card-image-wrapper">
                        <img src=image alt=name class="card-image" />
                        <div class="card-overlay" />
                    </div>
                    <div class="card-content">
                        <h3 class="card-title">{name}</h3>
                        <p class="card-description">{description}</p>
                        <span class="card-link">
                            "View Project"
                            <span class="link-arrow">"→"</span>
                        </span>
                    </div>
                </div>
            </a>
        </div>
    }
}

/// Projects section with 3D cards.
#[component]
fn ProjectsSection() -> impl IntoView {
    let is_visible = use_element_in_view("projects-section", 0.1);

    view! {
        <section
            id="projects-section"
            class="mobile-projects-3d"
            class:section-visible=move || is_visible.get()
        >
            <div class="section-header">
                <span class="section-label">"Featured Work"</span>
                <h2 class="section-title-3d">"Projects"</h2>
            </div>

            <div class="projects-container">
                {LOCATION_WORK.children.iter().enumerate().map(|(idx, project)| {
                    let description = project.children
                        .and_then(|children| children.iter()
                            .find(|c| c.name.ends_with(".txt"))
                            .and_then(|txt| txt.description)
                            .map(|desc| desc.first().copied().unwrap_or("")))
                        .unwrap_or("");

                    let image = project.children
                        .and_then(|children| children.iter()
                            .find(|c| c.name.ends_with(".png"))
                            .and_then(|img| img.image_url))
                        .unwrap_or("/public/images/project-1.png");

                    let link = project.children
                        .and_then(|children| children.iter()
                            .find(|c| c.name.ends_with(".com") || c.name.ends_with(".io"))
                            .and_then(|url| url.href))
                        .unwrap_or("#");

                    view! {
                        <ProjectCard3D
                            name=project.name
                            description=description
                            image=image
                            link=link
                            index=idx
                        />
                    }
                }).collect_view()}
            </div>
        </section>
    }
}

/// Floating skill badge.
#[component]
fn SkillBadge(name: &'static str, delay: usize) -> impl IntoView {
    view! {
        <span class="skill-badge-3d" style:animation-delay=format!("{}ms", delay * 50)>
            {name}
        </span>
    }
}

/// Skills section with floating badges.
#[component]
fn SkillsSection() -> impl IntoView {
    let is_visible = use_element_in_view("skills-section", 0.1);

    view! {
        <section
            id="skills-section"
            class="mobile-skills-3d"
            class:section-visible=move || is_visible.get()
        >
            <div class="section-header">
                <span class="section-label">"Expertise"</span>
                <h2 class="section-title-3d">"Skills"</h2>
            </div>

            <div class="skills-container">
                {TECH_STACK.iter().map(|category| {
                    view! {
                        <div class="skill-category-3d">
                            <h3 class="category-name">{category.category}</h3>
                            <div class="skill-badges">
                                {category.items.iter().enumerate().map(|(idx, skill)| {
                                    view! { <SkillBadge name=skill delay=idx /> }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}

/// Contact link component.
#[component]
fn ContactLink(
    href: &'static str,
    icon: &'static str,
    name: &'static str,
    index: usize,
) -> impl IntoView {
    let delay_ms = index * 100;

    view! {
        <div class="reveal-card contact-reveal" style:transition-delay=format!("{}ms", delay_ms)>
            <a href=href target="_blank" rel="noopener" class="contact-link-3d">
                <img src=icon alt=name class="contact-icon" />
                <span class="contact-name">{name}</span>
            </a>
        </div>
    }
}

/// Contact section with clean design.
#[component]
fn ContactSection() -> impl IntoView {
    let is_visible = use_element_in_view("contact-section", 0.1);

    view! {
        <section
            id="contact-section"
            class="mobile-contact-3d"
            class:section-visible=move || is_visible.get()
        >
            <div class="section-header">
                <span class="section-label">"Connect"</span>
                <h2 class="section-title-3d">"Get In Touch"</h2>
            </div>

            <p class="contact-text-3d">
                "Open to new opportunities and collaborations."
            </p>

            <div class="contact-links">
                {SOCIALS.iter().enumerate().map(|(idx, social)| {
                    view! {
                        <ContactLink
                            href=social.link
                            icon=social.icon
                            name=social.text
                            index=idx
                        />
                    }
                }).collect_view()}

                <div
                    class="reveal-card contact-reveal"
                    style:transition-delay=format!("{}ms", SOCIALS.len() * 100)
                >
                    <a href="mailto:ranimalach@gmail.com" class="contact-link-3d email-link">
                        <span class="email-icon">"✉"</span>
                        <span class="contact-name">"Email Me"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}

/// Main mobile portfolio component with 3D effects.
#[component]
pub fn MobilePortfolio() -> impl IntoView {
    let (scroll_y, _scroll_progress) = use_scroll();

    view! {
        <div class="mobile-portfolio-3d">
            <HeroSection scroll_y=scroll_y />
            <ProjectsSection />
            <SkillsSection />
            <ContactSection />

            <footer class="mobile-footer-3d">
                <p>"© 2024 Rani Malach"</p>
                <p class="footer-tech">"Built with Rust & Leptos"</p>
            </footer>
        </div>
    }
}
