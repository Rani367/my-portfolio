//! Photos/Gallery window component.

use leptos::prelude::*;
use crate::data::{GALLERY, PHOTOS_LINKS};

/// Photos content component showing gallery.
#[component]
pub fn PhotosContent() -> impl IntoView {

    view! {
        <div class="photos-sidebar">
            <div class="sidebar-section">
                <span class="sidebar-title">"Library"</span>
                <div class="sidebar-items">
                    {PHOTOS_LINKS.iter().map(|link| {
                        view! {
                            <div class="sidebar-item">
                                <img src=link.icon alt="" />
                                <span>{link.title}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
        <div class="photos-content">
            <div class="gallery-grid">
                {GALLERY.iter().map(|item| {
                    let img_url = item.img;
                    view! {
                        <div class="gallery-item">
                            <img src=img_url alt="Gallery image" />
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
