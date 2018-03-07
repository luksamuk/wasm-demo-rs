#![recursion_limit="2048"]

#[macro_use]
extern crate stdweb;

use stdweb::unstable::TryInto;
use stdweb::traits::IMouseEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{
    self,
    IEventTarget,
    INonElementParentNode,
    CanvasRenderingContext2d,
};
use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    KeyDownEvent,
    KeyUpEvent,
    KeyboardLocation,

    MouseButton,
    MouseDownEvent,
    MouseUpEvent,
    MouseMoveEvent
};
use std::f64::consts::PI;


/// Handles keyboard events.
fn on_key(key: &str, location: KeyboardLocation, is_pressed: bool) -> bool {
    let location = format!("{:?}", location);
    console!(log, "Key: ", key, ", location: ", location, ", pressed: ", is_pressed);
    
    true
}


/// Handles mouse presses (up and down).
fn on_mouse_click(btn: MouseButton, is_pressed: bool, pos: (f64, f64)) -> bool {
    let btn = format!("{:?}", btn);
    console!(log, "MPos: (", pos.0, ", ", pos.1, ") MBtn: ", btn, " pressed: ", is_pressed);
    true
}

/// Handles sole mouse movement, without presses.
fn on_mouse_move(pos: (f64, f64)) -> bool {
    console!(log, "MPos: (", pos.0, ", ", pos.1, ")");
    true
}



/// Draws a colored box.
fn draw_box(ctx: &CanvasRenderingContext2d, color: &str, pos: (f64, f64), size: (f64, f64)) {
    ctx.set_fill_style_color(color);
    ctx.fill_rect(pos.0, pos.1, size.0, size.1);
}

/// Draws a colored circle.
fn draw_circle(ctx: &CanvasRenderingContext2d, color: &str, pos: (f64, f64), radius: f64) {
    // Still needs to be done manually
    ctx.begin_path();
    ctx.set_fill_style_color(color);
    ctx.arc(pos.0, pos.1, radius, 0.0, PI * 2.0, false);
    //ctx.fill();
    js!{ @(no_return) @{ctx}.fill(); }; // Still waiting for implementation
    ctx.close_path();
}



fn main() {
    stdweb::initialize();

    // Retrieve canvas
    let canvas: CanvasElement = web::document().get_element_by_id("viewport")
        .unwrap()
        .try_into()
        .unwrap();

    // Retrieve context
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    
    // === Drawing tests ===
    
    // Draw three boxes
    draw_box(&ctx, "red",   (20.0, 20.0), (150.0, 100.0));
    draw_box(&ctx, "blue",  (40.0, 40.0), (150.0, 100.0));
    draw_box(&ctx, "green", (60.0, 60.0), (150.0, 100.0));

    // Draw a circle out of convenience
    draw_circle(&ctx, "white", (95.0, 70.0), 20.0);

    
    // === Event bindings ===
    
    // Keyboard
    web::window().add_event_listener(|event: KeyDownEvent| {
        if on_key(&event.key(), event.location(), true) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: KeyUpEvent| {
        if on_key(&event.key(), event.location(), false) {
            event.prevent_default();
        }
    });


    
    // Mouse
    web::window().add_event_listener(|event: MouseDownEvent| {
        if on_mouse_click(event.button(), true, (event.client_x() as f64,
                                                 event.client_y() as f64)) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: MouseUpEvent| {
        if on_mouse_click(event.button(), false, (event.client_x() as f64,
                                                  event.client_y() as f64)) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: MouseMoveEvent| {
        if on_mouse_move((event.client_x() as f64, event.client_y() as f64)) {
            event.prevent_default();
        }
    });

    
    
    stdweb::event_loop();
}
