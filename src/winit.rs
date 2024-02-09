use crate::CalloopData;
use smithay::{
    backend::winit::{self, WinitEvent, Error as WinitError},
    reexports::calloop::EventLoop,
};

pub fn winit_init(
    eloop: &mut EventLoop<CalloopData>,
    calloop_data: &mut CalloopData,
) -> Result<(), Box<dyn std::error::Error>> {
    let display_handle = &mut calloop_data.display_handle;
    let state = &mut calloop_data.state;

    // i have no idea why this doesn't work, it did before
    let a /*(mut backend, mut winit)*/ = winit::init(None);
    println!("{:?}", a);

    Ok(())
}
