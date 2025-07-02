use dyn_clone::{clone_trait_object, DynClone};
use std::fmt::Debug;

pub(crate) trait TileExtension: Debug + DynClone {}
clone_trait_object!(TileExtension);

#[derive(Debug, Clone)]
pub(crate) struct Abbey {}
impl TileExtension for Abbey {}
