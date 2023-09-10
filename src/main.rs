//! A mouse-controlled tiling wayland compositor.
//! Written by ChromaCat248.

extern crate smithay;

use smithay::wayland::compositor::compositor_init;
use smithay::reexports::wayland_server::Display;
use std::{thread, time};
//mod tiling;

#[tokio::main]
async fn main() {
    let mut display = Display::new();
    let comp = compositor_init(
        &mut display,
        |surface, dispatch_data| {
            println!("{:?}", dispatch_data);
            println!("{:?}", surface);
        },
        None
    );
    println!("{:?}", comp);

    // keep program running until killed
    loop {
        thread::sleep(time::Duration::from_millis(100000));
    }
}
