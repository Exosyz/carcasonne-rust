use dyn_clone::{clone_trait_object, DynClone};
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Edge {
    North,
    West,
    East,
    South,
}

#[derive(Debug, Clone)]
pub(crate) struct TileFeature {
    pub(crate) feature_type: Box<dyn TileFeatureType>,
    pub(crate) edges: Vec<Edge>,
    pub(crate) enhancement: Option<Box<dyn TileFeatureEnhancement>>,
}

pub(crate) trait TileFeatureType: Debug + DynClone + Any {}
clone_trait_object!(TileFeatureType);

#[derive(Debug, Clone)]
pub(crate) struct Town {}
impl TileFeatureType for Town {}

#[derive(Debug, Clone)]
pub(crate) struct Road {}
impl TileFeatureType for Road {}

pub(crate) trait TileFeatureEnhancement: Debug + DynClone + Any {}
clone_trait_object!(TileFeatureEnhancement);

#[derive(Debug, Clone)]
pub(crate) struct Shield {}
impl TileFeatureEnhancement for Shield {}
