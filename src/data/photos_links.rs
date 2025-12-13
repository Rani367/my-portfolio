//! Photos sidebar links configuration data.

/// Represents a sidebar link in the photos window.
#[derive(Clone, Debug, PartialEq)]
pub struct PhotoLink {
    pub id: u32,
    pub icon: &'static str,
    pub title: &'static str,
}

/// All sidebar links for the photos window.
pub const PHOTOS_LINKS: &[PhotoLink] = &[
    PhotoLink {
        id: 1,
        icon: "/public/icons/gicon1.svg",
        title: "Library",
    },
    PhotoLink {
        id: 2,
        icon: "/public/icons/gicon2.svg",
        title: "Memories",
    },
    PhotoLink {
        id: 3,
        icon: "/public/icons/file.svg",
        title: "Places",
    },
    PhotoLink {
        id: 4,
        icon: "/public/icons/gicon4.svg",
        title: "Favorites",
    },
    PhotoLink {
        id: 5,
        icon: "/public/icons/gicon5.svg",
        title: "People",
    },
];
