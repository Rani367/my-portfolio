//! Clock hook for displaying time in macOS format.
//!
//! This hook provides a formatted time string that updates every minute,
//! matching the macOS menu bar clock format.

use leptos::prelude::*;
use chrono::{Local, Timelike, Datelike};
use gloo_timers::callback::Interval;

/// Hook to get the current time formatted like macOS menu bar.
///
/// Returns a signal containing a formatted time string like "Fri Dec 13 2:30 PM".
/// The time updates every minute.
///
/// # Returns
/// A  with the formatted time.
pub fn use_clock() -> RwSignal<String> {
    let time = RwSignal::new(format_current_time());

    // Set up interval to update time
    Effect::new(move |_| {
        // Update immediately
        time.set(format_current_time());

        // Then update every minute
        let interval = Interval::new(60_000, move || {
            time.set(format_current_time());
        });

        // Note: .forget() keeps the interval alive. This is intentional because:
        // 1. The menu bar clock is always visible and never unmounts
        // 2. gloo_timers::Interval is not Send+Sync, so on_cleanup won't work
        // 3. For singleton components like this, the "leak" is the expected behavior
        interval.forget();
    });

    time
}

/// Hook to get time with seconds (updates every second).
///
/// This is more resource-intensive and should only be used when
/// second-level precision is needed.
///
/// # Returns
/// A  with the formatted time including seconds.
pub fn use_clock_with_seconds() -> RwSignal<String> {
    let time = RwSignal::new(format_current_time_with_seconds());

    Effect::new(move |_| {
        time.set(format_current_time_with_seconds());

        let interval = Interval::new(1_000, move || {
            time.set(format_current_time_with_seconds());
        });

        // Note: .forget() keeps the interval alive - see use_clock() for rationale
        interval.forget();
    });

    time
}

/// Format current time in macOS menu bar style.
/// Example: "Fri Dec 13 2:30 PM"
fn format_current_time() -> String {
    let now = Local::now();
    
    let weekday = match now.weekday() {
        chrono::Weekday::Mon => "Mon",
        chrono::Weekday::Tue => "Tue",
        chrono::Weekday::Wed => "Wed",
        chrono::Weekday::Thu => "Thu",
        chrono::Weekday::Fri => "Fri",
        chrono::Weekday::Sat => "Sat",
        chrono::Weekday::Sun => "Sun",
    };

    let month = match now.month() {
        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "???",
    };

    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();

    let (hour_12, period) = if hour == 0 {
        (12, "AM")
    } else if hour < 12 {
        (hour, "AM")
    } else if hour == 12 {
        (12, "PM")
    } else {
        (hour - 12, "PM")
    };

    format!("{} {} {} {}:{:02} {}", weekday, month, day, hour_12, minute, period)
}

/// Format current time with seconds.
/// Example: "Fri Dec 13 2:30:45 PM"
fn format_current_time_with_seconds() -> String {
    let now = Local::now();
    
    let weekday = match now.weekday() {
        chrono::Weekday::Mon => "Mon",
        chrono::Weekday::Tue => "Tue",
        chrono::Weekday::Wed => "Wed",
        chrono::Weekday::Thu => "Thu",
        chrono::Weekday::Fri => "Fri",
        chrono::Weekday::Sat => "Sat",
        chrono::Weekday::Sun => "Sun",
    };

    let month = match now.month() {
        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "???",
    };

    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();

    let (hour_12, period) = if hour == 0 {
        (12, "AM")
    } else if hour < 12 {
        (hour, "AM")
    } else if hour == 12 {
        (12, "PM")
    } else {
        (hour - 12, "PM")
    };

    format!("{} {} {} {}:{:02}:{:02} {}", weekday, month, day, hour_12, minute, second, period)
}

/// Get just the time portion formatted.
/// Example: "2:30 PM"
pub fn format_time_only() -> String {
    let now = Local::now();
    let hour = now.hour();
    let minute = now.minute();

    let (hour_12, period) = if hour == 0 {
        (12, "AM")
    } else if hour < 12 {
        (hour, "AM")
    } else if hour == 12 {
        (12, "PM")
    } else {
        (hour - 12, "PM")
    };

    format!("{}:{:02} {}", hour_12, minute, period)
}
