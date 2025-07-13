use dyn_clone::{clone_trait_object, DynClone};
use std::any::Any;
use std::fmt::Debug;

/// Represents one of the four edges of a tile.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Edge {
    /// Top edge of the tile.
    North,
    /// Left edge of the tile.
    West,
    /// Right edge of the tile.
    East,
    /// Bottom edge of the tile.
    South,
}

/// A feature present on a tile (e.g., town, road), possibly with enhancements.
///
/// A `TileFeature` defines:
/// - The type of the feature (such as a `Town` or `Road`)
/// - The edges of the tile that the feature touches
/// - An optional enhancement (like a `Shield`) that modifies scoring or rules
#[derive(Debug, Clone)]
pub struct TileFeature {
    /// The core type of the feature (e.g., town, road).
    pub feature_type: Box<dyn TileFeatureType>,
    /// The edges of the tile this feature spans.
    pub edges: Vec<Edge>,
    /// An optional enhancement that provides additional functionality or scoring.
    pub enhancement: Option<Box<dyn TileFeatureEnhancement>>,
}

/// Trait representing a type of tile feature (e.g., road, town, field).
///
/// This trait allows for dynamic dispatch and cloning of feature types.
pub trait TileFeatureType: Debug + DynClone + Any + Sync {}

// Enables cloning of `TileFeatureType` trait objects.
clone_trait_object!(TileFeatureType);

/// A concrete implementation of a tile feature: a town.
#[derive(Debug, Clone)]
pub struct Town {}
impl TileFeatureType for Town {}

/// A concrete implementation of a tile feature: a road.
#[derive(Debug, Clone)]
pub struct Road {}
impl TileFeatureType for Road {}

/// Trait representing an optional enhancement on a tile feature,
/// such as a shield in a town.
///
/// Enhancements may affect scoring or gameplay behavior.
pub trait TileFeatureEnhancement: Debug + DynClone + Any + Sync {}

// Enables cloning of `TileFeatureEnhancement` trait objects.
clone_trait_object!(TileFeatureEnhancement);

/// A specific enhancement that can be applied to a town: a shield.
///
/// Shields typically grant bonus points when features are scored.
#[derive(Debug, Clone)]
pub struct Shield {}
impl TileFeatureEnhancement for Shield {}
