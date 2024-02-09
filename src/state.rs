//! State struct

use std::{
    rc::Rc,
    cell::RefCell,
};

use smithay::reexports::{
    calloop::EventLoop,
    wayland_server::Display,
};

use crate::CalloopData;

pub struct ZlandState {
    display_handle: Rc<RefCell<Display>>, // this is DisplayHandle in later wayland_server releases
    
    // one tile tree globally will not work with multi-monitor setups, later when i'm past the
    // "just get it to work at all" phase i should change it to every screen having an associated
    // vec of workspaces, each having associated tile trees

    //tile_tree: Option<TileTreeNode>, //TODO after the compositor works
}

impl ZlandState {
    pub fn new(display: Rc<RefCell<Display>>, eloop: &mut EventLoop<CalloopData>) -> Self {
        Self {display_handle: display}
    }
}
