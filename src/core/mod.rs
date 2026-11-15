//! # core
//!
//! `core` contains the core game logic. This module is intended to
//! be imported by things that handle presentation.

/// The game object, keeps track of players and the grid.
pub mod game;

/// The grid keeps track of walls.
pub mod grid;

/// Enum used to distinguish between horizontal and vertical.
pub mod orientation;

/// Describes the placement of a wall.
pub mod placement;

/// The player object.
pub mod player;

/// Position is a generic x,y coordinate container.
pub mod position;

pub mod game_rounds_state;
pub mod player_turn;
/// A wall, used to stop players from advancing position.
pub mod wall;
