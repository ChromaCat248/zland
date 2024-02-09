//! A mouse-controlled tiling wayland compositor.
//! Written by ChromaCat248.

#![allow (dead_code, unused)]

use std::{rc::Rc, cell::RefCell};

use smithay::{
    wayland::compositor::compositor_init,
    reexports::{
        calloop::EventLoop,
        wayland_server::Display,
    },
};

use std::{thread, time};

mod tiling;
mod state;
mod winit;

use state::ZlandState;

#[derive(Debug)]
pub enum ZlandError {
    UnknownArg,
}

impl std::fmt::Display for ZlandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ZlandError::UnknownArg => "UnknownArg"
        })
    }
}

impl std::error::Error for ZlandError {}

pub struct CalloopData {
    state: ZlandState,
    display_handle: Rc<RefCell<Display>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
 
    // parse args
    // TODO after the compositor works
    /*
    let mut args = std::env::args();

    enum LastArg { StartCommand }
    use LastArg::*;
    let mut last_arg: Option<LastArg> = None;

    let mut unknown_arg = false;
    let mut start_command = "kitty".into(); // default init command inside this string

    for current_arg in args.skip(1) {
        //println!("arg: {}", current_arg);
        
        if let Some(x) = last_arg {
            match x {
                StartCommand => { start_command = current_arg },
                _ => {}
            }
            last_arg = None;
            continue;
        }

        last_arg = Some( match current_arg.as_ref() {
            "--start-command" => { println!("startcommand"); StartCommand },
            _ => {
                println!("unknown argument: {}", current_arg);
                unknown_arg = true;
                continue;
            },
        });
    };

    println!("{}", start_command);
    if unknown_arg { return Err(Box::new(ZlandError::UnknownArg)) }
    */

    // initialization
    let mut eloop: EventLoop<CalloopData> = EventLoop::try_new()?;
    let display = Rc::new(RefCell::new(Display::new()));
    let state = ZlandState::new(display.clone(), &mut eloop);

    let mut calloop_data = CalloopData {
        state,
        display_handle: display,
    };

    println!("fgdshiusdfg");
    winit::winit_init(&mut eloop, &mut calloop_data)?;

    println!("Initialization complete, starting main loop");
    eloop.run(None, &mut calloop_data, |_| {});

    Ok(())
}
