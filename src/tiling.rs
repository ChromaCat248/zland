//! This module defines the tile tree and its implementations.
//! TODO after the compositor works

pub enum TileTreeNode {
    Leaf {
        window: (),
        width: u32,
    },
    Branch {
        children: Vec<TileTreeNode>,
        width: u32,
    },
}
