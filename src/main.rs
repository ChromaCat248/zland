//! A mouse-controlled tiling wayland compositor.
//! Written by ChromaCat248.

extern crate smithay;

use smithay::wayland::compositor::compositor_init;
use smithay::reexports::wayland_server::Display;
use std::{thread, time};
//mod tiling;

fn main() {
    let mut display = Display::new();
    let socket = display.add_socket::<&str>(None);
    match socket {
        Ok(()) => println!("Socket created"),
        Err(error) => panic!("could not create socket: {} error", error.kind())
    };

    let (comp, subcomp) = compositor_init(
        &mut display,
        |surface, dispatch_data| {
            println!("{:?}", dispatch_data);
            println!("{:?}", surface);
        },
        None
    );
    println!("{:?} {:?}", comp, subcomp);

    // keep program running until killed
    loop {
        thread::sleep(time::Duration::from_millis(100000));
    }
}
