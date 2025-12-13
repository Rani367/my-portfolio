//! Terminal window component displaying skills.

use leptos::prelude::*;
use crate::data::TECH_STACK;

/// Terminal content component showing skills.
#[component]
pub fn TerminalContent() -> impl IntoView {
    view! {
        <div class="terminal-content">
            <div class="terminal-line">
                <span class="terminal-prompt">"portfolio@mac ~ %"</span>
                <span class="terminal-command">" cat skills.json"</span>
            </div>
            <div class="terminal-output">
                {TECH_STACK.iter().map(|category| {
                    let items_str = category.items.join(", ");
                    view! {
                        <div class="skill-category">
                            <span class="skill-category-name">{category.category}":"</span>
                            <div class="skill-items">{items_str}</div>
                        </div>
                    }
                }).collect_view()}
            </div>
            <div class="terminal-line">
                <span class="terminal-prompt">"portfolio@mac ~ %"</span>
                <span class="terminal-command terminal-cursor">" _"</span>
            </div>
        </div>
    }
}
