//! Gallery configuration data.

/// Represents a gallery image item.
#[derive(Clone, Debug, PartialEq)]
pub struct GalleryItem {
    pub id: u32,
    pub img: &'static str,
}

/// All gallery images for the photos window.
pub const GALLERY: &[GalleryItem] = &[
    GalleryItem {
        id: 1,
        img: "/public/images/gal1.png",
    },
    GalleryItem {
        id: 2,
        img: "/public/images/gal2.png",
    },
    GalleryItem {
        id: 3,
        img: "/public/images/gal3.png",
    },
    GalleryItem {
        id: 4,
        img: "/public/images/gal4.png",
    },
];
