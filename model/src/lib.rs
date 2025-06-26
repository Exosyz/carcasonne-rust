//! This module serves as the entry point for various components of the game application.
//! It consists of several submodules, each handling a specific aspect of the game's functionality:
//!
//! - `tile`: This module is responsible for defining and managing the properties and behavior of tiles
//!           that make up the game board.
//!
//! - `game`: This module encapsulates the core logic of the game, including initialization, progression,
//!           and termination of the game session.
//!
//! - `player`: This module manages player-related data and operations, such as player state, actions,
//!             and interactions within the game.
//!
//! - `scoreboard`: This module handles score tracking and management to record and display player
//!                 performance metrics throughout the game.
//!
//! - `board`: This module is responsible for constructing and maintaining the game board, managing
//!            its state, and providing utility functions to interact with it.
//!
//! - `pawn`: This module models the pawns or pieces used in the game, including their behavior,
//!           movement, and interactions with other elements.
//!
//! - `side`: This module represents the different sides or teams in the game, defining their
//!           characteristics and functionality.
//!
//! - `builder`: This module provides functionality to build and customize the game's components
//!              programmatically, facilitating flexibility in creating game variants.
//!
//! Each module in this structure plays a critical role in delivering a cohesive and fully functional
//! game implementation.
pub mod board;
pub mod builder;
pub mod game;
pub mod pawn;
pub mod player;
pub mod scoreboard;
pub mod side;
pub mod tile;
