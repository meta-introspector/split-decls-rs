use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait WalkableAstNode : Sized { type FlatMapOutputTy ; fn walk < D > (& mut self , _collector : & mut D) ; fn walk_flat_map < D > (self , _collector : & mut D) -> Self :: FlatMapOutputTy ; fn wrap_flat_map_node_walk_flat_map < D > (node : Self , _collector : & mut D , walk_flat_map : impl FnOnce (Self , & mut D) -> Self :: FlatMapOutputTy ,) -> Result < Self :: FlatMapOutputTy , Self > ; }
}