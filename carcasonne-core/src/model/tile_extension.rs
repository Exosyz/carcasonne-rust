use dyn_clone::{clone_trait_object, DynClone};
use std::any::Any;
use std::fmt::Debug;

/// A trait for extending the behavior or metadata of a tile.
///
/// `TileExtension` allows different tile types to provide custom logic
/// beyond the basic feature layout (e.g., special scoring rules, placement constraints).
///
/// It is a trait object (`dyn TileExtension`) that supports:
/// - dynamic dispatch,
/// - cloning (via [`DynClone`](https://docs.rs/dyn-clone)),
/// - and runtime type inspection (via [`Any`]).
///
/// # Example
///
/// ```
/// use carcasonne_core::model::tile_extension::TileExtension;
///
/// #[derive(Debug, Clone)]
/// struct Abbey;
///
/// impl TileExtension for Abbey {}
/// ```
pub trait TileExtension: Debug + DynClone + Any + Sync {}

// Enables cloning of trait objects for `TileExtension`.
clone_trait_object!(TileExtension);

/// A concrete implementation of `TileExtension` representing an Abbey tile.
///
/// This tile may have specific scoring or placement behavior,
/// which can be handled dynamically at runtime.
#[derive(Debug, Clone)]
pub struct Abbey {}
impl TileExtension for Abbey {}
