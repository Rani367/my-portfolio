//! Notification system state management.

use leptos::prelude::*;

/// A notification to display.
#[derive(Clone, Debug, PartialEq)]
pub struct Notification {
    /// Unique ID for the notification.
    pub id: u32,
    /// App icon path.
    pub icon: &'static str,
    /// App name.
    pub app_name: &'static str,
    /// Notification title.
    pub title: String,
    /// Notification body text.
    pub body: String,
    /// Timestamp string.
    pub time: String,
    /// Whether the notification is being dismissed.
    pub is_dismissing: bool,
}

impl Notification {
    /// Create a new notification.
    pub fn new(
        id: u32,
        icon: &'static str,
        app_name: &'static str,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id,
            icon,
            app_name,
            title: title.into(),
            body: body.into(),
            time: "now".to_string(),
            is_dismissing: false,
        }
    }
}

/// Notification state manager.
#[derive(Clone)]
pub struct NotificationState {
    /// List of active notifications.
    notifications: RwSignal<Vec<Notification>>,
    /// Counter for generating unique IDs.
    next_id: RwSignal<u32>,
}

impl NotificationState {
    /// Create a new notification state.
    pub fn new() -> Self {
        Self {
            notifications: RwSignal::new(Vec::new()),
            next_id: RwSignal::new(1),
        }
    }

    /// Show a new notification.
    pub fn show(&self, icon: &'static str, app_name: &'static str, title: impl Into<String>, body: impl Into<String>) -> u32 {
        let id = self.next_id.get();
        self.next_id.update(|n| *n += 1);
        
        let notification = Notification::new(id, icon, app_name, title, body);
        self.notifications.update(|list| {
            list.push(notification);
        });
        
        id
    }

    /// Show a system notification (uses Finder icon).
    pub fn show_system(&self, title: impl Into<String>, body: impl Into<String>) -> u32 {
        self.show("/public/images/finder.png", "System", title, body)
    }

    /// Start dismissing a notification.
    pub fn dismiss(&self, id: u32) {
        self.notifications.update(|list| {
            if let Some(notif) = list.iter_mut().find(|n| n.id == id) {
                notif.is_dismissing = true;
            }
        });
    }

    /// Remove a notification after dismiss animation.
    pub fn remove(&self, id: u32) {
        self.notifications.update(|list| {
            list.retain(|n| n.id != id);
        });
    }

    /// Get all notifications.
    pub fn get_all(&self) -> Vec<Notification> {
        self.notifications.get()
    }

    /// Clear all notifications.
    pub fn clear_all(&self) {
        self.notifications.update(|list| list.clear());
    }
}

impl Default for NotificationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide NotificationState to the component tree.
pub fn provide_notification_state() {
    let state = NotificationState::new();
    provide_context(state);
}

/// Get NotificationState from context.
pub fn use_notification_state() -> NotificationState {
    expect_context::<NotificationState>()
}
