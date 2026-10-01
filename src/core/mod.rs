//! # core
//!
//! `core` contains the core game logic. This module is intended to
//! be imported by things that handle presentation.

/// A block, used to stop players from advancing position.
pub mod wall;
pub mod game;
pub mod grid;
pub mod orientation;
pub mod placement;
pub mod player;
pub mod position;
mod tile;
