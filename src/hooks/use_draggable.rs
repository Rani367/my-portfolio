//! Draggable hook for window movement.
//!
//! This hook enables dragging functionality for windows by tracking mouse
//! movements and updating position signals accordingly.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, window};

/// Position tuple type (x, y).
pub type Position = (i32, i32);

/// Hook return type with position signal and drag start handler.
pub struct UseDraggableReturn {
    /// Current position signal.
    pub position: RwSignal<Position>,
    /// Whether currently dragging.
    pub is_dragging: RwSignal<bool>,
}

/// Create a draggable hook for window movement.
///
/// This hook sets up mouse event handlers for dragging. The returned
/// is_dragging signal can be used to apply dragging styles.
///
/// # Arguments
/// * initial_position - The initial (x, y) position of the element.
///
/// # Returns
/// A UseDraggableReturn containing position and dragging state signals.
pub fn use_draggable(initial_position: Position) -> UseDraggableReturn {
    let position = RwSignal::new(initial_position);
    let is_dragging = RwSignal::new(false);
    let drag_start = RwSignal::new((0_i32, 0_i32));
    let initial_pos = RwSignal::new((0_i32, 0_i32));

    // Set up global mouse event listeners
    Effect::new(move |_| {
        let win = window().expect("no global window exists");
        let document = win.document().expect("no document on window");

        // Mousemove handler
        let move_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
            if is_dragging.get() {
                let (start_x, start_y) = drag_start.get();
                let (init_x, init_y) = initial_pos.get();
                let new_x = init_x + e.client_x() - start_x;
                let new_y = (init_y + e.client_y() - start_y).max(28); // Keep below menu bar
                position.set((new_x, new_y));
            }
        }) as Box<dyn Fn(MouseEvent)>);

        // Mouseup handler
        let up_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
            is_dragging.set(false);
        }) as Box<dyn Fn(MouseEvent)>);

        document
            .add_event_listener_with_callback("mousemove", move_closure.as_ref().unchecked_ref())
            .expect("failed to add mousemove listener");
        document
            .add_event_listener_with_callback("mouseup", up_closure.as_ref().unchecked_ref())
            .expect("failed to add mouseup listener");

        // Keep closures alive
        move_closure.forget();
        up_closure.forget();
    });

    UseDraggableReturn {
        position,
        is_dragging,
    }
}

/// Start dragging from a mousedown event.
/// Call this from the header/drag handle of your window.
pub fn start_drag(
    e: MouseEvent,
    position: RwSignal<Position>,
    is_dragging: RwSignal<bool>,
    drag_start: RwSignal<Position>,
    initial_pos: RwSignal<Position>,
) {
    e.prevent_default();
    is_dragging.set(true);
    drag_start.set((e.client_x(), e.client_y()));
    initial_pos.set(position.get());
}

/// Create dragging state signals that can be passed to start_drag.
pub fn create_drag_state(initial_position: Position) -> (
    RwSignal<Position>,
    RwSignal<bool>,
    RwSignal<Position>,
    RwSignal<Position>,
) {
    let position = RwSignal::new(initial_position);
    let is_dragging = RwSignal::new(false);
    let drag_start = RwSignal::new((0_i32, 0_i32));
    let initial_pos = RwSignal::new((0_i32, 0_i32));

    // Set up global mouse event listeners
    Effect::new(move |_| {
        let win = window().expect("no global window exists");
        let document = win.document().expect("no document on window");

        let move_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
            if is_dragging.get() {
                let (start_x, start_y) = drag_start.get();
                let (init_x, init_y) = initial_pos.get();
                let new_x = init_x + e.client_x() - start_x;
                let new_y = (init_y + e.client_y() - start_y).max(28);
                position.set((new_x, new_y));
            }
        }) as Box<dyn Fn(MouseEvent)>);

        let up_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
            is_dragging.set(false);
        }) as Box<dyn Fn(MouseEvent)>);

        document
            .add_event_listener_with_callback("mousemove", move_closure.as_ref().unchecked_ref())
            .expect("failed to add mousemove listener");
        document
            .add_event_listener_with_callback("mouseup", up_closure.as_ref().unchecked_ref())
            .expect("failed to add mouseup listener");

        move_closure.forget();
        up_closure.forget();
    });

    (position, is_dragging, drag_start, initial_pos)
}
