use leptos::prelude::*;
use leptos::mount::mount_to_body;

pub mod data;
pub mod state;
pub mod hooks;
pub mod components;

use state::{provide_app_state, provide_navigation_state, provide_notification_state, use_app_state, WindowId};
use hooks::use_mobile_detection;
use components::{
    Window, MenuBar, Dock, FinderContent, TxtFileContent, ImgFileContent,
    TerminalContent, ContactContent, PhotosContent, AboutContent, SafariContent,
    ResumeContent, Spotlight, StartupScreen, NotificationCenter,
    MobileHomeScreen, MobileAppContainer,
};

#[component]
fn App() -> impl IntoView {
    // Provide global state
    provide_app_state();
    provide_navigation_state();
    provide_notification_state();

    let app_state = use_app_state();

    // Initialize mobile detection - this will set app_state.is_mobile
    let _device_type = use_mobile_detection();

    // Open Finder window on mount (desktop mode will show it, mobile will hide via CSS)
    app_state.open_window(WindowId::Finder);

    view! {
        // Apple startup screen
        <StartupScreen />

        // ========================================
        // DESKTOP UI (hidden on mobile via CSS)
        // ========================================

        // Menu bar at top
        <MenuBar />

        // Desktop area
        <div id="desktop" class="loaded">
            // Finder window
            <Window
                id=WindowId::Finder
                title="Finder"
                width=900
                height=550
                initial_top=60
                initial_left=100
                body_class="finder-body".to_string()
            >
                <FinderContent />
            </Window>

            // Terminal window
            <Window
                id=WindowId::Terminal
                title="Terminal — skills"
                width=700
                height=450
                initial_top=100
                initial_left=200
                body_class="terminal-body".to_string()
            >
                <TerminalContent />
            </Window>

            // Contact window
            <Window
                id=WindowId::Contact
                title="Contacts"
                width=600
                height=400
                initial_top=120
                initial_left=250
                body_class="contact-body".to_string()
            >
                <ContactContent />
            </Window>

            // Photos window
            <Window
                id=WindowId::Photos
                title="Photos"
                width=800
                height=500
                initial_top=80
                initial_left=150
                body_class="photos-body".to_string()
            >
                <PhotosContent />
            </Window>

            // Safari window
            <Window
                id=WindowId::Safari
                title="Safari"
                width=900
                height=600
                initial_top=50
                initial_left=120
                body_class="safari-body".to_string()
            >
                <SafariContent />
            </Window>

            // Text file viewer window
            <Window
                id=WindowId::TxtFile
                title="TextEdit"
                width=600
                height=450
                initial_top=80
                initial_left=200
                body_class="txtfile-body".to_string()
            >
                <TxtFileContent />
            </Window>

            // Image file viewer window
            <Window
                id=WindowId::ImgFile
                title="Preview"
                width=700
                height=500
                initial_top=70
                initial_left=180
                body_class="imgfile-body".to_string()
            >
                <ImgFileContent />
            </Window>

            // About This Mac window
            <Window
                id=WindowId::About
                title=""
                width=300
                height=400
                initial_top=100
                initial_left=300
                body_class="about-body".to_string()
            >
                <AboutContent />
            </Window>

            // Resume PDF viewer window
            <Window
                id=WindowId::Resume
                title="Resume.pdf"
                width=650
                height=700
                initial_top=40
                initial_left=200
                body_class="resume-body".to_string()
            >
                <ResumeContent />
            </Window>
        </div>

        // Dock at bottom
        <Dock />

        // Spotlight search overlay
        <Spotlight />

        // ========================================
        // MOBILE UI (iOS Mode)
        // ========================================

        // iOS-style home screen with app grid
        <MobileHomeScreen />

        // Mobile app containers (full-screen with swipe to close)
        <MobileAppContainer id=WindowId::Finder title="Portfolio">
            <FinderContent />
        </MobileAppContainer>

        <MobileAppContainer id=WindowId::Terminal title="Skills">
            <TerminalContent />
        </MobileAppContainer>

        <MobileAppContainer id=WindowId::Contact title="Contact">
            <ContactContent />
        </MobileAppContainer>

        <MobileAppContainer id=WindowId::Photos title="Gallery">
            <PhotosContent />
        </MobileAppContainer>

        <MobileAppContainer id=WindowId::Safari title="Safari">
            <SafariContent />
        </MobileAppContainer>

        <MobileAppContainer id=WindowId::Resume title="Resume">
            <ResumeContent />
        </MobileAppContainer>

        <MobileAppContainer id=WindowId::About title="About">
            <AboutContent />
        </MobileAppContainer>

        <MobileAppContainer id=WindowId::TxtFile title="TextEdit">
            <TxtFileContent />
        </MobileAppContainer>

        <MobileAppContainer id=WindowId::ImgFile title="Preview">
            <ImgFileContent />
        </MobileAppContainer>

        // Notification center (works on both mobile and desktop)
        <NotificationCenter />
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
