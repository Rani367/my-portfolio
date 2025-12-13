//! Text file viewer component.

use leptos::prelude::*;
use crate::state::use_app_state;

/// Text file viewer content component.
#[component]
pub fn TxtFileContent() -> impl IntoView {
    let app_state = use_app_state();

    // Get the current txt file data
    let file_data = Memo::new(move |_| {
        app_state.txt_file_data.get()
    });

    view! {
        <div class="txt-content">
            {move || {
                if let Some(data) = file_data.get() {
                    let file = data.file;

                    // Build the content
                    let header_view = if file.subtitle.is_some() && file.image.is_some() {
                        Some(view! {
                            <div class="txt-header">
                                <img src=file.image.unwrap_or_default() alt="" class="txt-header-image" />
                                <div class="txt-header-content">
                                    <div class="txt-subtitle">{file.subtitle.unwrap_or_default()}</div>
                                </div>
                            </div>
                        })
                    } else {
                        None
                    };

                    let description_view = file.description.map(|desc| {
                        desc.iter().map(|para| {
                            view! { <p>{*para}</p> }
                        }).collect_view()
                    });

                    view! {
                        {header_view}
                        {description_view}
                    }.into_any()
                } else {
                    view! {
                        <p>"No file selected"</p>
                    }.into_any()
                }
            }}
        </div>
    }
}
