//! Image file viewer component.

use leptos::prelude::*;
use crate::state::use_app_state;

/// Image file viewer content component.
#[component]
pub fn ImgFileContent() -> impl IntoView {
    let app_state = use_app_state();

    // Get the current img file data
    let file_data = Memo::new(move |_| {
        app_state.img_file_data.get()
    });

    view! {
        {move || {
            if let Some(data) = file_data.get() {
                let file = data.file;
                let image_url = file.image_url.unwrap_or_default();

                view! {
                    <img
                        src=image_url
                        alt=file.name
                        class="img-preview"
                    />
                }.into_any()
            } else {
                view! {
                    <div class="img-placeholder">"No image selected"</div>
                }.into_any()
            }
        }}
    }
}
