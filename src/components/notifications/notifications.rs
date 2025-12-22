//! Notification center component.

use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;

use crate::state::{use_notification_state, Notification};

/// Individual notification item.
#[component]
fn NotificationItem(notification: Notification) -> impl IntoView {
    let notification_state = use_notification_state();
    let id = notification.id;

    let (is_visible, set_is_visible) = signal(false);
    let (is_hiding, set_is_hiding) = signal(false);

    // Show animation on mount
    spawn_local(async move {
        TimeoutFuture::new(10).await;
        set_is_visible.set(true);
    });

    // Auto-dismiss after 5 seconds - checks if already hiding before proceeding
    {
        let notification_state = notification_state.clone();
        spawn_local(async move {
            TimeoutFuture::new(5000).await;
            // Only proceed if not already dismissed by user click
            if !is_hiding.get_untracked() {
                set_is_hiding.set(true);
                TimeoutFuture::new(400).await;
                notification_state.remove(id);
            }
        });
    }

    // Handle click to dismiss
    let on_click = {
        let notification_state = notification_state.clone();
        move |_| {
            set_is_hiding.set(true);
            let notification_state = notification_state.clone();
            spawn_local(async move {
                TimeoutFuture::new(400).await;
                notification_state.remove(id);
            });
        }
    };

    let class = move || {
        let mut classes = vec!["notification"];
        if is_visible.get() && !is_hiding.get() {
            classes.push("show");
        }
        if is_hiding.get() {
            classes.push("hide");
        }
        classes.join(" ")
    };

    view! {
        <div class=class on:click=on_click>
            <div class="notification-header">
                <img src=notification.icon alt="" class="notification-icon" />
                <span class="notification-app">{notification.app_name}</span>
                <span class="notification-time">{notification.time.clone()}</span>
            </div>
            <div class="notification-title">{notification.title.clone()}</div>
            <div class="notification-body">{notification.body.clone()}</div>
        </div>
    }
}

/// Notification center that displays all notifications.
#[component]
pub fn NotificationCenter() -> impl IntoView {
    let notification_state = use_notification_state();

    // Show welcome notification after startup (3.5s delay)
    {
        let notification_state = notification_state.clone();
        spawn_local(async move {
            TimeoutFuture::new(3500).await;
            notification_state.show(
                "/public/images/finder.png",
                "Portfolio",
                "Welcome!",
                "Click around to explore my portfolio.\nUse the search icon in the menu bar for Spotlight.",
            );
        });
    }

    view! {
        <div id="notification-center">
            <For
                each=move || notification_state.get_all()
                key=|notification| notification.id
                let:notification
            >
                <NotificationItem notification=notification />
            </For>
        </div>
    }
}
