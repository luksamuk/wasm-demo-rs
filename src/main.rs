#![recursion_limit="2048"]

#[macro_use]
extern crate stdweb;

use stdweb::web::{
    self,
    IEventTarget,
    INode,
    IElement,
    FileReader,
    FileReaderResult,
    Element,
    ArrayBuffer
};

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    ClickEvent,
    ChangeEvent,
    ProgressLoadEvent,
    KeydownEvent,
    KeyupEvent,
    KeyboardLocation
};


fn on_key(key: &str, location: KeyboardLocation, is_pressed: bool) -> bool {
    let location = format!("{:?}", location);
    js!( console.log("Key: " + @{key} + ", location: " + @{location} + ", pressed: " + @{is_pressed}); );
    true
}


fn main() {
    stdweb::initialize();

    let canvas = web::document().get_element_by_id("viewport").unwrap();
    let ctx: stdweb::Value = js!( return @{canvas}.getContext("2d"); ); 
    
    let draw_box = |ctx: &stdweb::Value, color: &str, pos: (f32, f32), sz: (f32, f32)| {
        js!(
            @{ctx}.beginPath();
            @{ctx}.rect(@{pos.0}, @{pos.1}, @{sz.0}, @{sz.1});
            @{ctx}.fillStyle = @{color};
            @{ctx}.fill();
        );
    };

    draw_box(&ctx, "red",   (20.0, 20.0), (150.0, 100.0));
    draw_box(&ctx, "blue",  (40.0, 40.0), (150.0, 100.0));
    draw_box(&ctx, "green", (60.0, 60.0), (150.0, 100.0));

    web::window().add_event_listener(|event: KeydownEvent| {
        if on_key(&event.key(), event.location(), true) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: KeyupEvent| {
        if on_key(&event.key(), event.location(), false) {
            event.prevent_default();
        }
    });
    
    stdweb::event_loop();
}
